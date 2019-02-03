use dom::events::*;
use dom::types::*;
use dom::node::*;
use dom::tree::*;

use jss::types::*;

pub fn get_sample_dom_tree() -> DOMTree<BasicEvent> {
    #[rustfmt::skip]
    let container_style = StyleBuilder::default()
        .case(Case::Ignore)
        .parse_from_str(r#"{
            "justify-content": "space-between",
            "background": "rgba(0,0,0,0.3)",
            "flex-direction": "row",
            "align-items": "center",

            "padding-left": "20px",
            "padding-right": "20px",

            "border-top-left-radius": "15px",
            "border-top-right-radius": "15px",
            "border-bottom-left-radius": "15px",
            "border-bottom-right-radius": "15px"
        }"#)
        .unwrap();

    #[rustfmt::skip]
    let item_style = StyleBuilder::default()
        .case(Case::Ignore)
        .parse_from_str(r#"{
            "justify-content": "space-between",
            "background": "rgb(3,169,244)",
            "align-items": "center",
            "margin-top": "10px",
            "height": "250px",
            "width": "250px",

            "border-top-color": "rgba(0,0,0,0.6)",
            "border-top-width": 10,

            "border-top-left-radius": "10px",
            "border-top-right-radius": "10px",
            "border-bottom-left-radius": "10px",
            "border-bottom-right-radius": "10px"
        }"#)
        .unwrap();

    let tree: DOMTree<BasicEvent> = {
        let mut fragment = DOMTree::default();

        {
            let mut parent = fragment.root_mut();
            {
                let mut parent = parent.append(DOMNode::from((
                    DOMTagName::from(KnownElementName::Div),
                    vec![DOMAttribute::from((
                        DOMAttributeName::from("name"),
                        DOMAttributeValue::from("body"),
                    ))],
                    container_style,
                )));

                {
                    let mut first_item = parent.append(DOMNode::from((
                        DOMTagName::from(KnownElementName::Div),
                        vec![DOMAttribute::from((
                            DOMAttributeName::from("name"),
                            DOMAttributeValue::from("item"),
                        ))],
                        item_style.clone(),
                    )));
                }

                {
                    let mut second_item = parent.append(DOMNode::from((
                        DOMTagName::from(KnownElementName::Div),
                        vec![DOMAttribute::from((
                            DOMAttributeName::from("name"),
                            DOMAttributeValue::from("item"),
                        ))],
                        item_style.clone(),
                    )));
                }

                {
                    let mut three_item = parent.append(DOMNode::from((
                        DOMTagName::from(KnownElementName::Div),
                        vec![DOMAttribute::from((
                            DOMAttributeName::from("name"),
                            DOMAttributeValue::from("item"),
                        ))],
                        item_style.clone(),
                    )));
                }
            }
        }

        fragment
    };

    tree
}
