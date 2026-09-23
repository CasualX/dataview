use dataview::Pod;

#[derive(Pod)]
#[repr(C)]
struct OldType(u32);

#[test]
fn pod_is_shared_with_1_1() {
    fn requires_new_pod<T: dataview_1_1::Pod>(_: &T) {}
    let value = OldType(0x1234_5678);
    requires_new_pod(&value);

    let new_view = dataview_1_1::DataView::from(&value);
    assert_eq!(new_view.read::<u32>(0), value.0);

    let old_view = dataview::DataView::from(&value);
    assert_eq!(old_view.read::<u32>(0), value.0);

    let new_only_type = dataview_1_1::DataView::from(&value);
    assert_eq!(dataview::DataView::from(new_only_type).read::<u32>(0), value.0);
    assert_eq!(dataview_1_1::DataView::from(old_view).read::<u32>(0), value.0);
}
