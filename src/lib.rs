/* In our application, all users have a name.
However they can be in two states:
  * deleted, in which case they have a deletion date
  * active, in which case they have a list of subscriptions
*/

use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub struct Name(String);
#[derive(Debug, PartialEq, Eq)]
pub struct Subscription(String);

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name(value.to_string())
    }
}

impl From<&str> for Subscription {
    fn from(value: &str) -> Self {
        Subscription(value.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UserState {
    Active,
    Deleted,
}

/* LAB STARTS HERE */

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    /* add anything you want, but make sure that invalid states are unrepresentable:
        - all states the User struct can be in correspond to a valid state
        - all valid states have a unique representation
    */
}

impl User {
    /* create a new active user */
    pub fn new(name: Name, subscriptions: Vec<Subscription>) -> Self {
        todo!()
    }

    /* delete an user.
       You can create an instant by using:
       let now = Instant::now();
    */
    pub fn delete(self) -> Option<Self> {
        todo!()
    }

    /* get the user state */
    pub fn state(&self) -> UserState {
        todo!()
    }

    /* accessor for name */
    pub fn name(&self) -> &Name {
        todo!()
    }

    /* accessor for deletion time */
    pub fn deleted_on(&self) -> Option<Instant> {
        todo!()
    }

    /* accessor for subscruption */
    pub fn subscriptions(&self) -> Option<&[Subscription]> {
        todo!()
    }

    /* add a subscription */
    pub fn subscribe(self, sub: Subscription) -> Option<Self> {
        todo!()
    }
}

/*  The list of subscriptions must never be empty. We also want to traverse them like a "cursor", where there is a current subscription, and be can go forward and backwards.

    For example, let's say there are five subscriptions, and the current subscription is c. We will use '*' to mark the current subscription. In pseudo code:

    s = [a, b, *c, d, e]
    s.current() == c
    s.next() == Some(d), and s = [a, b, c, *d, e]
    s.next() == Some(e), and s = [a, b, c, d, *e]
    s.next() == None, and s = [a, b, c, d, *e]

    Fill out the following struct so that the fact that the list of subscriptions is non-empty is statically enforced.
    Bonus points if the operations are efficient!
*/

#[derive(Debug)]
pub struct Subscriptions {/* TODO */}

impl Subscriptions {
    pub fn new(sub: Subscription) -> Self {
        todo!()
    }

    /* retrieves the current subscription */
    pub fn current(&self) -> &Subscription {
        todo!()
    }

    /* get to the next subscription, and, if possible, return a reference to it */
    pub fn next_sub(&mut self) -> Option<&Subscription> {
        todo!()
    }

    /* get to the previous subscription, and, if possible, return a reference to it */
    pub fn prev_sub(&mut self) -> Option<&Subscription> {
        todo!()
    }

    /* add a new subsubscription *right after the current subscription* */
    pub fn add(&mut self, sub: Subscription) {
        todo!()
    }

    /* turn into a list */
    pub fn to_list(&self) -> Vec<&Subscription> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active() {
        let u = User::new("Bob".into(), vec!["sub1".into()]);
        assert_eq!(u.state(), UserState::Active);
        assert_eq!(u.deleted_on(), None);
        assert_eq!(u.subscriptions(), Some(vec!["sub1".into()].as_slice()));
        let u = u.subscribe("sub2".into()).unwrap();
        assert_eq!(u.subscriptions(), Some(vec!["sub1".into(), "sub2".into()].as_slice()));
    }

    #[test]
    fn deleted() {
        let u = User::new("Bob".into(), vec!["sub1".into()]);
        let u = u.delete().unwrap();
        assert_eq!(u.state(), UserState::Deleted);
        assert!(u.deleted_on().is_some());
        assert_eq!(u.subscriptions(), None);
        assert_eq!(u.subscribe("x".into()), None);
    }

    #[test]
    fn double_delete_impossible() {
        let u = User::new("Bob".into(), vec!["sub1".into()]);
        let u = u.delete().unwrap();
        assert_eq!(u.delete(), None);
    }

    #[test]
    fn subs1() {
        let mut s = Subscriptions::new("a".into());
        assert_eq!(s.current().0, "a");
        assert_eq!(s.next_sub(), None);
        assert_eq!(s.current().0, "a");
        let mut lst: Vec<Subscription> = vec!["a".into()];
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        s.add("b".into());
        lst.push("b".into());
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.current().0, "a");
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.next_sub(), Some(&"b".into()));
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.current().0, "b");
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.next_sub(), None);
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        s.add("c".into());
        lst.push("c".into());
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.current().0, "b");
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.next_sub(), Some(&"c".into()));
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.prev_sub(), Some(&"b".into()));
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.prev_sub(), Some(&"a".into()));
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.prev_sub(), None);
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        assert_eq!(s.current().0, "a");
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
        s.add("d".into());
        lst.insert(1, "d".into());
        assert_eq!(s.to_list(), lst.iter().collect::<Vec<_>>());
    }
}
