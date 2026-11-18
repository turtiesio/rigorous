//! An organisation: the six institutional units composed in parallel, plus a standard rule set.
//!
//! [`org`] is used unchanged for a team, a company, or a company nested inside another
//! (wrap it in an `Entity`): the same semantics at every scale.

use crate::kernel::{Cell, Par, Unit};
use crate::logic::datalog::{Atom, Literal, Rule, Term, Var};
use crate::model::Fault;
use crate::model::{
    ActorId, ConsExpr, Const, Event, EvidenceRules, Finding, GovernanceRules, Kind, MemberKind, Method, NormRules, PartyExpr, Pred, RLiteral, RRule, RTerm,
    Rank, ResponsibilityRules, Role, RuleId, RulePath, Rules, Status, Tag, Template, Trust, VerdictTemplate, ViolationTemplate,
};
use crate::units::{adjudicator::Adjudicator, epistemic::Epistemic, governance::Governance, institution::Institution, ledger::Ledger, norms::Norms};
use std::collections::BTreeSet;

/// Kinds any unit may emit: they are authenticated by the kernel's actor stamp, not by ownership.
#[must_use]
pub fn open_kinds() -> BTreeSet<Kind> {
    [Kind::Do, Kind::Signal].into_iter().collect()
}

/// The institutional units plus the given member units.
///
/// # Errors
/// If the members clash with the institution (names or output ownership).
pub fn org(members: Vec<Box<dyn Unit<Event>>>) -> Result<Par<Event>, Fault> {
    let mut parts: Vec<Box<dyn Unit<Event>>> = vec![
        Box::new(Cell(Institution::default())),
        Box::new(Cell(Ledger::default())),
        Box::new(Cell(Epistemic::default())),
        Box::new(Cell(Norms::default())),
        Box::new(Cell(Adjudicator::default())),
        Box::new(Cell(Governance::default())),
    ];
    parts.extend(members);
    Par::new(parts, &open_kinds())
}

// ---------------------------------------------------------------- rule-writing helpers

const X: RTerm = Term::Var(Var(0));
const Y: RTerm = Term::Var(Var(1));
const C: RTerm = Term::Var(Var(2));
const D: RTerm = Term::Var(Var(3));
const P: RTerm = Term::Var(Var(4));
const Q: RTerm = Term::Var(Var(5));

fn pos<const N: usize>(pred: Pred, args: [RTerm; N]) -> RLiteral {
    Literal::Pos(Atom { pred, args: args.into() })
}
fn neg<const N: usize>(pred: Pred, args: [RTerm; N]) -> RLiteral {
    Literal::Neg(Atom { pred, args: args.into() })
}
fn rule<const N: usize>(id: RuleId, pred: Pred, head: [RTerm; N], body: Vec<RLiteral>) -> RRule {
    Rule { id, head: Atom { pred, args: head.into() }, body }
}
const VIOLATED: RTerm = Term::Const(Const::Status(Status::Violated));
const AI: RTerm = Term::Const(Const::Kind(MemberKind::Ai));
const FALSE: RTerm = Term::Const(Const::Bool(false));

/// Responsibility rules, version 1. Read `Pred(x, c)` as "x is <pred> for commitment c".
#[must_use]
pub fn responsibility_v1() -> Vec<RRule> {
    use Pred::{
        Accountable, AtFault, Claimed, Cons, Contributed, Culpable, Debtor, Excused, HasSubFailure, IsAi, Kind as KindOf, Knew, Liable, Misreported, Notified,
        Parent, Principal, Status as StatusOf, SubFailed,
    };
    vec![
        // The debtor of a violated commitment answers for it, whatever it delegated.
        rule(const { RuleId::lit("acc") }, Accountable, [X, C], vec![pos(StatusOf, [C, VIOLATED]), pos(Debtor, [C, X])]),
        rule(const { RuleId::lit("sub") }, SubFailed, [C, D], vec![pos(Parent, [D, C]), pos(StatusOf, [D, VIOLATED])]),
        rule(const { RuleId::lit("hsub") }, HasSubFailure, [C], vec![pos(SubFailed, [C, D])]),
        rule(const { RuleId::lit("knew") }, Knew, [X, C], vec![pos(Debtor, [C, X]), pos(SubFailed, [C, D]), pos(Cons, [D, Q]), pos(Notified, [X, Q, FALSE])]),
        rule(
            const { RuleId::lit("mis") },
            Misreported,
            [X, C],
            vec![pos(StatusOf, [C, VIOLATED]), pos(Debtor, [C, X]), pos(Cons, [C, P]), pos(Claimed, [X, P])],
        ),
        rule(const { RuleId::lit("culp") }, Culpable, [X, C], vec![pos(Misreported, [X, C]), pos(Knew, [X, C])]),
        rule(const { RuleId::lit("exc") }, Excused, [X, C], vec![pos(Accountable, [X, C]), pos(SubFailed, [C, D]), neg(Misreported, [X, C])]),
        rule(const { RuleId::lit("contr") }, Contributed, [Y, C], vec![pos(SubFailed, [C, D]), pos(Debtor, [D, Y])]),
        rule(const { RuleId::lit("flt1") }, AtFault, [X, C], vec![pos(Culpable, [X, C])]),
        rule(const { RuleId::lit("flt2") }, AtFault, [X, C], vec![pos(Accountable, [X, C]), neg(HasSubFailure, [C])]),
        rule(const { RuleId::lit("ai") }, IsAi, [X], vec![pos(KindOf, [X, AI])]),
        // An AI agent can be at fault but cannot bear liability: it lifts to its principal.
        rule(const { RuleId::lit("liab1") }, Liable, [Y, C], vec![pos(AtFault, [X, C]), pos(KindOf, [X, AI]), pos(Principal, [X, Y])]),
        rule(const { RuleId::lit("liab2") }, Liable, [X, C], vec![pos(AtFault, [X, C]), neg(IsAi, [X])]),
    ]
}

/// The amendment used in the demonstrator: a duty to verify before reporting success.
#[must_use]
pub fn duty_to_verify() -> Vec<RRule> {
    vec![
        rule(const { RuleId::lit("negl") }, Pred::Negligent, [X, C], vec![pos(Pred::Misreported, [X, C]), neg(Pred::Knew, [X, C])]),
        rule(const { RuleId::lit("flt3") }, Pred::AtFault, [X, C], vec![pos(Pred::Negligent, [X, C])]),
    ]
}

/// Who the standard rule set trusts for what.
#[derive(Clone, Debug)]
pub struct Trusted {
    pub receipts: ActorId,
    pub probes: ActorId,
    pub tests: ActorId,
    pub audits: ActorId,
    pub acks: ActorId,
}

/// The standard rule set, version 1.
#[must_use]
pub fn standard_rules(t: &Trusted) -> Rules {
    let only = |a: &ActorId| Trust::Only(std::iter::once(a.clone()).collect());
    let tags = |ts: &[Tag]| ts.iter().copied().collect::<BTreeSet<Tag>>();
    Rules {
        evidence: EvidenceRules {
            ranks: [(Method::Receipt, 1), (Method::Ack, 2), (Method::Test, 2), (Method::Probe, 3), (Method::Audit, 4), (Method::Inference, 9)]
                .into_iter()
                .map(|(m, r)| (m, Rank(r)))
                .collect(),
            trusted: [
                (Method::Receipt, only(&t.receipts)),
                (Method::Probe, only(&t.probes)),
                (Method::Test, only(&t.tests)),
                (Method::Audit, only(&t.audits)),
                (Method::Ack, only(&t.acks)),
                (Method::Inference, Trust::Anyone),
            ]
            .into_iter()
            .collect(),
            self_support: false,
        },
        norms: NormRules {
            on_violation: vec![
                ViolationTemplate {
                    tag: Tag::Task,
                    create: Template {
                        debtor: PartyExpr::RoleHolder(Role::Auditor),
                        creditor: PartyExpr::Creditor,
                        cons: ConsExpr::Audited,
                        within: 10,
                        tags: tags(&[Tag::Audit]),
                    },
                },
                // A standing duty that lapsed must be put right, and that is tracked as a duty of its own.
                ViolationTemplate {
                    tag: Tag::Upkeep,
                    create: Template {
                        debtor: PartyExpr::Debtor,
                        creditor: PartyExpr::Creditor,
                        cons: ConsExpr::Remediated,
                        within: 7,
                        tags: tags(&[Tag::Remediation]),
                    },
                },
            ],
            on_verdict: vec![
                VerdictTemplate {
                    finding: Finding::Liable,
                    tag: Tag::Task,
                    create: Template {
                        debtor: PartyExpr::Party,
                        creditor: PartyExpr::Creditor,
                        cons: ConsExpr::Remediated,
                        within: 5,
                        tags: tags(&[Tag::Remediation]),
                    },
                },
                VerdictTemplate {
                    finding: Finding::Liable,
                    tag: Tag::Paid,
                    create: Template {
                        debtor: PartyExpr::Party,
                        creditor: PartyExpr::Payer,
                        cons: ConsExpr::RefundOfPrice,
                        within: 5,
                        tags: tags(&[Tag::Refund]),
                    },
                },
            ],
            maintenance_period: 30 * 24 * 3600,
        },
        responsibility: ResponsibilityRules { rules: responsibility_v1(), findings: Finding::ALL.into_iter().collect(), retroactive: true },
        governance: GovernanceRules { quorum: 2, entrenched: [RulePath::Entrenched, RulePath::SelfSupport].into_iter().collect() },
    }
}
