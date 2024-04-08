use super::*;

#[test]
fn intos() {
    let val = "Random String".to_owned();
    let cw: CW<_> = val.clone().into();
    let arc = Arc::new(val.clone());
    {
        let actual_arc = cw.clone().into();
        assert_eq!(arc, actual_arc);
        assert_eq!(Arc::strong_count(&arc), 1);
        assert_eq!(Arc::strong_count(cw.inner()), 2);
        assert_eq!(Arc::strong_count(&actual_arc), 2);
    }
    assert_eq!(Arc::strong_count(cw.inner()), 1);
    {
        let actual_cw = CW::from(arc);
        assert_eq!(cw, actual_cw);
        assert_eq!(Arc::strong_count(actual_cw.inner()), 1);
    }
    {
        let actual_val = cw.take();
        assert_eq!(val, actual_val);
    }
}

#[test]
fn clone() {
    #[derive(Debug, Eq, PartialEq)]
    struct NotClone;

    let cw: CW<_> = NotClone.into();
    let cw_clone = cw.clone();
    assert_eq!(cw_clone, cw)
}
