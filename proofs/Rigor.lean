/-
  Machine-checked core lemmas (Lean 4, core library only; no Mathlib).
  Check with:   lean proofs/Rigor.lean

  These are proofs about the MATHEMATICAL MODEL in docs/02-semantics.md.
  They are not proofs about the Python code; the correspondence between the
  two is established only by tests (see docs/03-status-ledger.md).
-/

namespace Rigor

/-! ## 1. Units and parallel composition -/

/-- A deterministic open transition system over events `E`. -/
structure Unit (E : Type) where
  S    : Type
  init : S
  step : S → E → S × List E

/-- Parallel composition: every event is delivered to both parts. -/
def par {E : Type} (A B : Unit E) : Unit E where
  S    := A.S × B.S
  init := (A.init, B.init)
  step := fun s e =>
    let a := A.step s.1 e
    let b := B.step s.2 e
    ((a.1, b.1), a.2 ++ b.2)

/-- State reached after a sequence of events. -/
def run {E : Type} (U : Unit E) : U.S → List E → U.S
  | s, []      => s
  | s, e :: es => run U (U.step s e).1 es

/-- An invariant: holds initially and is preserved by every step. -/
structure Inv {E : Type} (U : Unit E) (P : U.S → Prop) : Prop where
  base : P U.init
  ind  : ∀ s e, P s → P (U.step s e).1

theorem inv_run {E : Type} (U : Unit E) (P : U.S → Prop) (h : Inv U P) :
    ∀ es, P (run U U.init es) := by
  have gen : ∀ es s, P s → P (run U s es) := by
    intro es
    induction es with
    | nil => intro s hs; exact hs
    | cons e es ih => intro s hs; exact ih _ (h.ind s e hs)
  intro es; exact gen es _ h.base

/-- T1 (local reasoning is sound): local invariants, each checked on its own unit,
    hold jointly on the composite.  No product state space is ever enumerated. -/
theorem inv_par {E : Type} (A B : Unit E) (P : A.S → Prop) (Q : B.S → Prop)
    (hA : Inv A P) (hB : Inv B Q) :
    Inv (par A B) (fun s => P s.1 ∧ Q s.2) :=
  ⟨⟨hA.base, hB.base⟩, fun s e h => ⟨hA.ind s.1 e h.1, hB.ind s.2 e h.2⟩⟩

/-- A forward simulation with identical outputs: `A` refines `B`. -/
structure Sim {E : Type} (A B : Unit E) (R : A.S → B.S → Prop) : Prop where
  base : R A.init B.init
  ind  : ∀ a b e, R a b → R (A.step a e).1 (B.step b e).1 ∧ (A.step a e).2 = (B.step b e).2

/-- T2 (refinement is compositional): refine the parts independently and the
    composite refines the composite.  This is what licenses substitution. -/
theorem sim_par {E : Type} (A A' B B' : Unit E)
    (R : A.S → A'.S → Prop) (T : B.S → B'.S → Prop)
    (hA : Sim A A' R) (hB : Sim B B' T) :
    Sim (par A B) (par A' B') (fun s s' => R s.1 s'.1 ∧ T s.2 s'.2) := by
  refine ⟨⟨hA.base, hB.base⟩, ?_⟩
  intro a b e h
  have h1 := hA.ind a.1 b.1 e h.1
  have h2 := hB.ind a.2 b.2 e h.2
  refine ⟨⟨h1.1, h2.1⟩, ?_⟩
  show (A.step a.1 e).2 ++ (B.step a.2 e).2 = (A'.step b.1 e).2 ++ (B'.step b.2 e).2
  rw [h1.2, h2.2]

/-- Outputs of a run (the observable trace). -/
def trace {E : Type} (U : Unit E) : U.S → List E → List E
  | _, []      => []
  | s, e :: es => (U.step s e).2 ++ trace U (U.step s e).1 es

/-- T3: a simulation implies trace equality on every input sequence. -/
theorem sim_trace {E : Type} (A B : Unit E) (R : A.S → B.S → Prop) (h : Sim A B R) :
    ∀ es, trace A A.init es = trace B B.init es := by
  have gen : ∀ es a b, R a b → trace A a es = trace B b es := by
    intro es
    induction es with
    | nil => intro a b _; rfl
    | cons e es ih =>
      intro a b r
      have hs := h.ind a b e r
      show (A.step a e).2 ++ trace A (A.step a e).1 es = (B.step b e).2 ++ trace B (B.step b e).1 es
      rw [hs.2, ih _ _ hs.1]
  intro es; exact gen es _ _ h.base

/-- T4 (associativity): the state bracketing is the only difference between
    (A ∥ B) ∥ C and A ∥ (B ∥ C); steps and outputs agree exactly. -/
theorem par_assoc {E : Type} (A B C : Unit E) :
    Sim (par (par A B) C) (par A (par B C))
      (fun s t => s.1.1 = t.1 ∧ s.1.2 = t.2.1 ∧ s.2 = t.2.2) := by
  refine ⟨⟨rfl, rfl, rfl⟩, ?_⟩
  intro a b e h
  obtain ⟨h1, h2, h3⟩ := h
  obtain ⟨⟨a1, a2⟩, a3⟩ := a
  obtain ⟨b1, b2, b3⟩ := b
  simp only at h1 h2 h3
  subst h1; subst h2; subst h3
  exact ⟨⟨rfl, rfl, rfl⟩, by simp [par, List.append_assoc]⟩

/-! ## 2. Independence: events with disjoint footprints commute -/

/-- A unit that ignores events outside its footprint `acc`. -/
def Ignores {E : Type} (U : Unit E) (acc : E → Prop) : Prop :=
  ∀ s e, ¬ acc e → (U.step s e).1 = s

/-- T5 (the diamond): if `e1` is only in A's footprint and `e2` only in B's, the
    two orders reach the same composite state.  This is the independence relation
    behind partial-order (Mazurkiewicz-trace) execution. -/
theorem commute {E : Type} (A B : Unit E) (accA accB : E → Prop)
    (hA : Ignores A accA) (hB : Ignores B accB)
    (e1 e2 : E) (h1 : ¬ accB e1) (h2 : ¬ accA e2) (s : (par A B).S) :
    run (par A B) s [e1, e2] = run (par A B) s [e2, e1] := by
  obtain ⟨a, b⟩ := s
  simp only [run, par]
  rw [hB b e1 h1, hA a e2 h2, hA _ e2 h2, hB _ e1 h1]

/-! ## 3. Delegation can only attenuate -/

/-- A capability: a set of permitted actions and a spending limit. -/
structure Cap (Act : Type) where
  may   : Act → Prop
  limit : Nat

/-- `c` attenuates `p`: no new action, no higher limit. -/
def Attenuates {Act : Type} (c p : Cap Act) : Prop :=
  (∀ a, c.may a → p.may a) ∧ c.limit ≤ p.limit

/-- A delegation chain from a root, each link attenuating the previous one. -/
inductive Chain {Act : Type} (root : Cap Act) : Cap Act → Prop
  | root : Chain root root
  | link : ∀ {p c}, Chain root p → Attenuates c p → Chain root c

/-- T6: however long the chain and whoever is on it, the capability at its end
    grants nothing the root did not grant. -/
theorem no_amplification {Act : Type} (root c : Cap Act) (h : Chain root c) :
    (∀ a, c.may a → root.may a) ∧ c.limit ≤ root.limit := by
  induction h with
  | root => exact ⟨fun _ h => h, Nat.le_refl _⟩
  | link _ att ih => exact ⟨fun a ha => ih.1 a (att.1 a ha), Nat.le_trans att.2 ih.2⟩

/-! ## 4. Transfers conserve resources -/

/-- Balances over a finite list of holders. -/
def total {H : Type} (bal : H → Nat) : List H → Nat
  | []      => 0
  | h :: hs => bal h + total bal hs

def upd {H : Type} [DecidableEq H] (bal : H → Nat) (x : H) (v : Nat) : H → Nat :=
  fun h => if h = x then v else bal h

theorem total_upd_notin {H : Type} [DecidableEq H] (bal : H → Nat) (x : H) (v : Nat) :
    ∀ hs : List H, x ∉ hs → total (upd bal x v) hs = total bal hs := by
  intro hs
  induction hs with
  | nil => intro _; rfl
  | cons h hs ih =>
    intro hx
    have hne : h ≠ x := fun e => hx (e ▸ List.mem_cons_self)
    have hin : x ∉ hs := fun m => hx (List.mem_cons_of_mem _ m)
    simp [total, upd, hne, ih hin]

theorem total_upd_in {H : Type} [DecidableEq H] (bal : H → Nat) (x : H) (v : Nat) :
    ∀ hs : List H, hs.Nodup → x ∈ hs → total (upd bal x v) hs + bal x = total bal hs + v := by
  intro hs
  induction hs with
  | nil => intro _ hx; cases hx
  | cons h hs ih =>
    intro nd hx
    have ⟨hnot, nd'⟩ := List.nodup_cons.mp nd
    by_cases e : h = x
    · subst e
      have := total_upd_notin bal h v hs hnot
      simp [total, upd, this]; omega
    · have hin : x ∈ hs := by
        cases hx with
        | head => exact absurd rfl e
        | tail _ m => exact m
      have := ih nd' hin
      simp [total, upd, e]; omega

/-- A transfer of `n` from `a` to `b` (enabled only when `a` can afford it). -/
def transfer {H : Type} [DecidableEq H] (bal : H → Nat) (a b : H) (n : Nat) : H → Nat :=
  upd (upd bal a (bal a - n)) b (upd bal a (bal a - n) b + n)

/-- T7 (conservation): an enabled transfer between two distinct holders leaves
    the total unchanged. -/
theorem transfer_conserves {H : Type} [DecidableEq H] (bal : H → Nat) (hs : List H)
    (nd : hs.Nodup) (a b : H) (n : Nat) (ha : a ∈ hs) (hb : b ∈ hs) (hab : a ≠ b)
    (hn : n ≤ bal a) : total (transfer bal a b n) hs = total bal hs := by
  have h1 := total_upd_in bal a (bal a - n) hs nd ha
  have h2 := total_upd_in (upd bal a (bal a - n)) b (upd bal a (bal a - n) b + n) hs nd hb
  have hbb : upd bal a (bal a - n) b = bal b := by simp [upd, Ne.symm hab]
  unfold transfer
  rw [hbb] at h2 ⊢
  omega

end Rigor

#print axioms Rigor.inv_par
#print axioms Rigor.sim_par
#print axioms Rigor.sim_trace
#print axioms Rigor.par_assoc
#print axioms Rigor.commute
#print axioms Rigor.no_amplification
#print axioms Rigor.transfer_conserves
