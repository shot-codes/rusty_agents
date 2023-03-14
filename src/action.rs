#[derive(Debug)]
pub enum ActionType {
    NoOp,
    Move,
    Push,
    Pull,
}

#[derive(Debug)]
pub struct Action<'a> {
    pub name: &'a str,
    pub action_type: ActionType,
    pub agent_row_delta: i32, // vertical displacement of agent
    pub agent_col_delta: i32, // horisontal displacement of agent
    pub box_row_delta: i32,   // vertical diplacement of box
    pub box_col_delta: i32,   // horisontal displacement of box
}

impl<'a> Action<'a> {
    pub const NO_OP: Action<'a> = Action {
        name: "NoOp",
        action_type: ActionType::NoOp,
        agent_row_delta: 0,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: 0,
    };

    pub const MOVE_N: Action<'a> = Action {
        name: "Move(N)",
        action_type: ActionType::Move,
        agent_row_delta: -1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: 0,
    };

    pub const MOVE_S: Action<'a> = Action {
        name: "Move(S)",
        action_type: ActionType::Move,
        agent_row_delta: 1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: 0,
    };

    pub const MOVE_E: Action<'a> = Action {
        name: "Move(E)",
        action_type: ActionType::Move,
        agent_row_delta: 0,
        agent_col_delta: 1,
        box_row_delta: 0,
        box_col_delta: 0,
    };

    pub const MOVE_W: Action<'a> = Action {
        name: "Move(W)",
        action_type: ActionType::Move,
        agent_row_delta: 0,
        agent_col_delta: -1,
        box_row_delta: 0,
        box_col_delta: 0,
    };

    pub const PUSH_NS: Action<'a> = Action {
        name: "Push(N,S)",
        action_type: ActionType::Push,
        agent_row_delta: -1,
        agent_col_delta: 0,
        box_row_delta: 1,
        box_col_delta: 0,
    };

    pub const PUSH_NN: Action<'a> = Action {
        name: "Push(N,N)",
        action_type: ActionType::Push,
        agent_row_delta: -1,
        agent_col_delta: 0,
        box_row_delta: -1,
        box_col_delta: 0,
    };

    pub const PUSH_NE: Action<'a> = Action {
        name: "Push(N,E)",
        action_type: ActionType::Push,
        agent_row_delta: -1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: 1,
    };

    pub const PUSH_NW: Action<'a> = Action {
        name: "Push(N,W)",
        action_type: ActionType::Push,
        agent_row_delta: -1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: -1,
    };

    pub const PUSH_SS: Action<'a> = Action {
        name: "Push(S,S)",
        action_type: ActionType::Push,
        agent_row_delta: 1,
        agent_col_delta: 0,
        box_row_delta: 1,
        box_col_delta: 0,
    };

    pub const PUSH_SN: Action<'a> = Action {
        name: "Push(S,N)",
        action_type: ActionType::Push,
        agent_row_delta: 1,
        agent_col_delta: 0,
        box_row_delta: -1,
        box_col_delta: 0,
    };

    pub const PUSH_SE: Action<'a> = Action {
        name: "Push(S,E)",
        action_type: ActionType::Push,
        agent_row_delta: 1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: 1,
    };

    pub const PUSH_SW: Action<'a> = Action {
        name: "Push(S,W)",
        action_type: ActionType::Push,
        agent_row_delta: 1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: -1,
    };

    pub const PUSH_ES: Action<'a> = Action {
        name: "Push(S,W)",
        action_type: ActionType::Push,
        agent_row_delta: 1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: -1,
    };

    pub const PUSH_EN: Action<'a> = Action {
        name: "Push(E,N)",
        action_type: ActionType::Push,
        agent_row_delta: 0,
        agent_col_delta: 1,
        box_row_delta: -1,
        box_col_delta: 0,
    };

    pub const PUSH_EE: Action<'a> = Action {
        name: "Push(E,E)",
        action_type: ActionType::Push,
        agent_row_delta: 0,
        agent_col_delta: 1,
        box_row_delta: 0,
        box_col_delta: 1,
    };

    pub const PUSH_EW: Action<'a> = Action {
        name: "Push(E,W)",
        action_type: ActionType::Push,
        agent_row_delta: 0,
        agent_col_delta: 1,
        box_row_delta: 0,
        box_col_delta: -1,
    };

    pub const PUSH_WS: Action<'a> = Action {
        name: "Push(W,S)",
        action_type: ActionType::Push,
        agent_row_delta: 0,
        agent_col_delta: -1,
        box_row_delta: 1,
        box_col_delta: 0,
    };

    pub const PUSH_WN: Action<'a> = Action {
        name: "Push(W,N)",
        action_type: ActionType::Push,
        agent_row_delta: 0,
        agent_col_delta: -1,
        box_row_delta: -1,
        box_col_delta: 0,
    };

    pub const PUSH_WE: Action<'a> = Action {
        name: "Push(W,E)",
        action_type: ActionType::Push,
        agent_row_delta: 0,
        agent_col_delta: -1,
        box_row_delta: 0,
        box_col_delta: 1,
    };

    pub const PUSH_WW: Action<'a> = Action {
        name: "Push(W,W)",
        action_type: ActionType::Push,
        agent_row_delta: 0,
        agent_col_delta: -1,
        box_row_delta: 0,
        box_col_delta: -1,
    };

    pub const PULL_NS: Action<'a> = Action {
        name: "Pull(N,S)",
        action_type: ActionType::Pull,
        agent_row_delta: -1,
        agent_col_delta: 0,
        box_row_delta: 1,
        box_col_delta: 0,
    };

    pub const PULL_NN: Action<'a> = Action {
        name: "Pull(N,N)",
        action_type: ActionType::Pull,
        agent_row_delta: -1,
        agent_col_delta: 0,
        box_row_delta: -1,
        box_col_delta: 0,
    };

    pub const PULL_NE: Action<'a> = Action {
        name: "Pull(N,E)",
        action_type: ActionType::Pull,
        agent_row_delta: -1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: 1,
    };

    pub const PULL_NW: Action<'a> = Action {
        name: "Pull(N,W)",
        action_type: ActionType::Pull,
        agent_row_delta: -1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: -1,
    };

    pub const PULL_ES: Action<'a> = Action {
        name: "Pull(E,S)",
        action_type: ActionType::Pull,
        agent_row_delta: 0,
        agent_col_delta: 1,
        box_row_delta: 1,
        box_col_delta: 0,
    };

    pub const PULL_EN: Action<'a> = Action {
        name: "Pull(E,N)",
        action_type: ActionType::Pull,
        agent_row_delta: 0,
        agent_col_delta: 1,
        box_row_delta: -1,
        box_col_delta: 0,
    };

    pub const PULL_SE: Action<'a> = Action {
        name: "Pull(S,E)",
        action_type: ActionType::Pull,
        agent_row_delta: 1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: 1,
    };

    pub const PULL_SW: Action<'a> = Action {
        name: "Pull(S,W)",
        action_type: ActionType::Pull,
        agent_row_delta: 1,
        agent_col_delta: 0,
        box_row_delta: 0,
        box_col_delta: -1,
    };

    pub const PULL_WN: Action<'a> = Action {
        name: "Pull(W,N)",
        action_type: ActionType::Pull,
        agent_row_delta: 0,
        agent_col_delta: -1,
        box_row_delta: -1,
        box_col_delta: 0,
    };

    pub const PULL_WE: Action<'a> = Action {
        name: "Pull(W,E)",
        action_type: ActionType::Pull,
        agent_row_delta: 0,
        agent_col_delta: -1,
        box_row_delta: 0,
        box_col_delta: 1,
    };

    pub const PULL_WW: Action<'a> = Action {
        name: "Pull(W,W)",
        action_type: ActionType::Pull,
        agent_row_delta: 0,
        agent_col_delta: -1,
        box_row_delta: 0,
        box_col_delta: -1,
    };
}
