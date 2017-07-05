extern crate lua;

#[derive(PartialEq)]
struct Data {
  value: String,
}

#[test]
fn test_extra_owned() {
  let mut state = lua::State::new();

  let data = Data {
    value: "Initial data".to_owned(),
  };
  state.set_extra(Some(Box::new(data)));

  for x in 0..10 {
    state.with_extra(|extra| {
      let data = extra.as_mut().unwrap()
        .downcast_mut::<Data>().unwrap();
      data.value = format!("Changed to {}", x);
    });
  }

  let extra = state.set_extra(None);
  let data = extra.as_ref().unwrap()
    .downcast_ref::<Data>().unwrap();
  assert_eq!(data.value, "Changed to 9");
}

#[test]
fn test_extra_typed() {
  let mut state = lua::State::new();

  let data = Data {
    value: "Initial data".to_owned(),
  };
  state.set_extra(Some(Box::new(data)));

  state.with_extra_typed(|data: &mut Data| {
    data.value = format!("Use typed");
  });

  let extra = state.set_extra(None).unwrap();
  let data = extra.downcast::<Data>().unwrap();
  assert_eq!(data.value, "Use typed");
}

#[test]
fn test_extra_threads() {
  let mut state = lua::State::new();

  let data = Data {
    value: "Initial data".to_owned(),
  };
  state.set_extra(Some(Box::new(data)));

  let mut thread = state.new_thread();
  let value = thread.with_extra(|extra| {
    let data = extra.as_ref().unwrap()
      .downcast_ref::<Data>().unwrap();
    data.value.clone()
  });
  assert_eq!(value, "Initial data");

  let data = Data {
    value: "Thread data".to_owned(),
  };
  thread.set_extra(Some(Box::new(data)));

  let value = state.with_extra(|extra| {
    let data = extra.as_ref().unwrap()
      .downcast_ref::<Data>().unwrap();
    data.value.clone()
  });
  assert_eq!(value, "Thread data");
}
