impl Document {
    pub fn new(data: &String) -> Self {
        Document::Dom(Dom::parse(&data).unwrap())
    }


    //loop over all the elements and call the provided function for each
    pub fn for_each<F: FnMut(&Document) -> bool>(&self, mut function: F) {
        // create a stack to create recursion alternative
        let mut stack = Vec::<Action<(Document, usize)>>::new();
        use Action::*;
        stack.push(Call((self.clone(), 0)));

        while let Some(action) = stack.pop() {
            match action {
                Call((document, level)) => {
                    match document {
                        Document::Element(element) => {
                            stack.push(Action::Handle((Document::Element(element.to_owned()), level)));
                            for document in &element.children {
                                match document {
                                    Node::Element(element) => {
                                        let document = Document::Element(element.to_owned());
                                        stack.push(Action::Call((document, level+1)));
                                    },
                                    _ => {}
                                }
                            }
                        },
                        Document::Dom(document) => {
                            for document in &document.children {
                                match document {
                                    Node::Element(element) => {
                                        let document = Document::Element(element.to_owned());
                                        stack.push(Action::Call((document, level+1)));
                                    },
                                    _ => {}
                                }
                            }
                        },
                        Document::Text(content) => {
                            let document = Document::Text(content);
                            stack.push(Action::Call((document, level+1)));
                        }
                    }
                },
                Handle((document, _)) => {
                    let out = function(&document);
                    if out {
                        return
                    }
                }
            }
        }
    }

    //get all elements with the provided id
    pub fn get_all_by_id(&self, id: Option<String>) -> Vec<Document> {
        let mut documents: Vec<Document> = Vec::new();

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    if element.id == id {
                        documents.push(document.clone());
                    };

                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        documents
    }

    //get element with the provided id
    pub fn get_by_id(&self, id: Option<String>) -> Option<Document> {
        let mut res: Option<Document> = None;
        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    if element.id == id {
                        res = Some(document.clone());
                        return true
                    };

                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        res
    }

    //get all elements that have the same classes as the provided
    pub fn get_all_by_class(&self, class: &String) -> Vec<Document> {
        let mut documents: Vec<Document> = Vec::new();

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    let classes: Vec<&str> = class.split(" ").collect();
                    if classes.len() != element.classes.len() {
                        return false
                    }
                    for index in 0..element.classes.len() {
                        if element.classes[index] != classes[index] {
                            return false
                        };
                    }

                    documents.push(document.clone());
                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        documents
    }

    //get element that has the same classes as the provided
    pub fn get_by_class(&self, class: &String) -> Option<Document> {
        let mut res: Option<Document> = None;

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    let classes: Vec<&str> = class.split(" ").collect();
                    if classes.len() != element.classes.len() {
                        return false
                    }
                    for index in 0..element.classes.len() {
                        if element.classes[index] != classes[index] {
                            return false
                        };
                    }

                    res = Some(document.clone());
                    return true
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        res
    }

    // get the first element that has the attribute provided 
    pub fn get_by_attribute(&self, attribute: &Attribute) -> Option<Document> {
        let mut res: Option<Document> = None;

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    if element.attributes.get(&attribute.key) == Some(&attribute.value) {
                        res = Some(document.clone());
                        return true
                    }

                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        res
    }

    // get all the elements that have the attribute provided 
    pub fn get_all_by_attribute(&self, attribute: &Attribute) -> Vec<Document> {
        let mut documents: Vec<Document> = Vec::new();

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    if element.attributes.get(&attribute.key) == Some(&attribute.value) {
                        documents.push(document.clone());
                    }

                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        documents
    }

    pub fn child_attribute(&self, key: &String) -> Option<String> {
        let mut value: Option<String> = None;

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    value = match element.attributes.get(key).clone() {
                        None => None,
                        Some(value) => value.clone()
                    };
                    if value !=  None {
                        return true
                    }
                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        value
    }

    pub fn child_attributes(&self, key: &String) -> Vec<String> {
        let mut attributes: Vec<String> = Vec::new();

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    let value = match element.attributes.get(key).clone() {
                        None => None,
                        Some(value) => value.clone()
                    };
                    if value !=  None {
                        attributes.push(value.unwrap());
                    }
                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        attributes
    }

    pub fn get_by_name(&self, name: &String) -> Option<Document> {
        let mut value: Option<Document> = None;

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    if element.name == *name {
                        value = Some(document.clone());
                        return true
                    }
                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        value
    }

    pub fn get_all_by_name(&self, name: &String) -> Vec<Document> {
        let mut documents = Vec::new();

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    if element.name == *name {
                        documents.push(document.clone());
                    }
                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        documents
    }

    pub fn get_text(&self) -> Option<String> {
        match self {
            Document::Text(text) => {
                return Some(text.clone())
            },
            Document::Element(element) => {
                for node in &element.children {
                    match node {
                        Node::Text(text) => {
                            return Some(text.clone())
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }

        None
    }

    pub fn attribute(&self, key: &String) -> Option<String> {
        let mut value: Option<String> = None;

        let function = |document: &Document| {
            match document {
                Document::Element(element) => {
                    value = element.attributes.get(key).unwrap().clone();
                    if value != None {
                        return true
                    }
                    return false
                },
                _ => {false},
            }
        };

        Document::for_each(self, function);
        value
    }
}