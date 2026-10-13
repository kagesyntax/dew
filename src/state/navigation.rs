//! "Return point": remembers exactly where the user was in a lesson before jumping into the
//! Bible, so one tap brings them back to the same lesson at the same scroll position instead
//! of the home screen.

use crate::routes::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnPoint {
    /// Short label for the pill, e.g. "Lesson 7".
    pub label: String,
    pub route: Route,
    pub scroll_y: f64,
}

/// Capture the current scroll offset, remember `route`, then navigate to `target`.
pub fn jump_with_return(mut state: AppState, label: String, route: Route, target: Route) {
    let nav = navigator();
    spawn(async move {
        let scroll_y = crate::platform::js::scroll_y().await;
        state.return_point.set(Some(ReturnPoint { label, route, scroll_y }));
        nav.push(target);
    });
}

/// Go back to the saved point and restore its scroll position.
pub fn go_back(mut state: AppState) {
    let point = state.return_point.write().take();
    if let Some(point) = point {
        state.pending_scroll.set(Some(point.scroll_y));
        navigator().push(point.route);
    }
}
