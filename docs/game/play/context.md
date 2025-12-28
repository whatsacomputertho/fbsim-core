# Context module

The `context` module includes the `PlayContext` struct which represents a game situation from the perspective of the offense. It is primarily used in the internals of the `PlayCallSimulator` to generate an offensive play call. It implements the `From` trait for converting `GameContext -> PlayContext`.
