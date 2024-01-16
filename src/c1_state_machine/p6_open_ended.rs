use super::StateMachine;

enum DayOfWeek {
	Thursday,
	Wednesday,
	Friday,
}

fn get_day_of_week() -> DayOfWeek {
	DayOfWeek::Wednesday
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Identity {
	RationalUtilityMaximizer,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Assignment {
	BlockchainFromScratch,
	AssignmentTwo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Track {
	Founder,
	Developer,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Activity {
	Code { assignment: Assignment },
	AttendEvent,
	DayDream { topic: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
	personal_identity: Identity,
	track: Track,
	activity: Activity,
}

pub enum Transition {
	DecideOnNextCourseOfAction,
	Code,
	DayDream,
}

impl StateMachine for State {
	type State = State;
	type Transition = Transition;

	fn next_state(initial_state: &Self::State, transition: &Self::Transition) -> Self::State {
		let mut output_state = initial_state.clone();

		match transition {
			Transition::DecideOnNextCourseOfAction => {
				if initial_state.personal_identity == Identity::RationalUtilityMaximizer
					&& initial_state.track == Track::Developer
				{
					output_state.activity =
						Activity::Code { assignment: Assignment::BlockchainFromScratch };
				} else {
					output_state.activity = Activity::AttendEvent
				}
			},
			Transition::Code => match get_day_of_week() {
				DayOfWeek::Wednesday => {
					output_state.activity =
						Activity::Code { assignment: Assignment::BlockchainFromScratch }
				},
				DayOfWeek::Thursday | DayOfWeek::Friday => {
					output_state.activity = Activity::Code { assignment: Assignment::AssignmentTwo }
				},
				_ => {
					output_state.activity =
						Activity::DayDream { topic: String::from("Next Big Idea") }
				},
			},
			Transition::DayDream => {
				output_state.activity = Activity::DayDream {
					topic: String::from("Find optimal use-case for Substrate"),
				};
			},
			_ => {},
		}
		output_state
	}
}
