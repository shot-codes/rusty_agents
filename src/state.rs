use crate::action::Action;
use crate::agent::Agent;
use crate::matrix::Matrix;

#[derive(Debug)]
pub struct State<'a> {
    pub agents: Vec<Agent>,
    pub goals: Vec<char>,
    pub walls: Matrix<bool>,
    pub boxes: Matrix<char>,
    pub parent: Option<Box<State<'a>>>,
    pub joint_action: Vec<Action<'a>>,
}
