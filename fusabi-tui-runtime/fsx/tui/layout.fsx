#load "rect.fsx"

// TUI Layout Types
// Represents layout constraints for dividing space

type Constraint =
    | Percentage of int
    | Length of int
    | Min of int
    | Max of int
    | Fill of int

type Direction = Horizontal | Vertical

// Note: Since Fusabi doesn't support record types or list types in DUs yet,
// we use a simple tuple representation
// Layout represented as direction * margin (constraints omitted)
type Layout =
    | Layout of Direction * int

// Layout constructors and utilities
let horizontalLayout constraints = Layout (Horizontal, 0)

let verticalLayout constraints = Layout (Vertical, 0)

let layoutWithMargin m layout =
    match layout with
    | Layout (dir, _) -> Layout (dir, m)

// split implementation (simplified placeholder)
let splitLayout layout area = []
