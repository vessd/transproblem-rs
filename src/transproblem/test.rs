use super::*;

fn init() -> Transportation {
    let a = vec![300, 250, 200];
    let b = vec![220, 150, 250, 180];
    let c = vec![vec![4, 5, 3, 6], vec![7, 2, 1, 5], vec![6, 1, 4, 2]];
    let t = match Transportation::new(&a, &b, &c) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };
    t
}

#[test]
fn test_transportation_create() {
    let t = init();
    assert_eq!(t.supply.len(), 4);
    assert_eq!(t.demand.len(), 4);
    assert_eq!(t.trans.data.len(), 16);
}

#[test]
fn test_least_cost_method() {
    let mut t = init();
    let result = vec![Some(170), None, None, Some(130), None, None, Some(250), None, None,
                      Some(150), None, Some(50), Some(50), None, None, None];
    t.least_cost_method();
    assert_eq!(t.trans.data, result);
}

#[test]
fn test_cycle_detection() {
    let mut t = init();
    t.least_cost_method();
    assert_eq!(t.cycle_detection(0, 0)[0][0], Direction::None);
    t.trans[0][1] = Some(0);
    assert_eq!(t.cycle_detection(0, 1)[0][1], Direction::Left);
}

#[test]
fn test_replenish() {
    let mut t = init();
    t.least_cost_method();
    t.replenish();
    assert_eq!(t.trans[0][2], Some(0));
}

#[test]
fn test_calculation_of_potentials() {
    let mut t = init();
    t.least_cost_method();
    t.replenish();
    let u = vec![0, -2, -4, -4];
    let v = vec![4, 5, 3, 6];
    assert_eq!(t.calculation_of_potentials(), (u, v));
}

#[test]
fn test_check() {
    let mut t = init();
    t.least_cost_method();
    t.replenish();
    assert_eq!(t.check(), Some((3, 3, -2)));
}

#[test]
fn test_total_cost() {
    let mut t = init();
    t.least_cost_method();
    assert_eq!(t.total_cost(), 1960);
}

#[test]
fn test_potential_method() {
    let mut t = init();
    t.potential_method();
    let result = vec![Some(220), None, Some(80), None, None, Some(80), Some(170), None, None,
                      Some(70), None, Some(130), None, None, None, Some(50)];
    assert_eq!(t.trans.data, result);
}

