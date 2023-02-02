# Tutorials

Tutorials exist on pages, each tutorial has some number of items, and each
item can be shown to the user. In addition, there should be some button available
to restart the tutorial for a given page.

Using the 'View Role' page as an example, we want to draw the user's attention to
some number of components of the page - The first is the information about who the
role is and the description, then comes the puzzle list, and after that comes the
permalink and shortcut copy buttons.

If we were to render that as Rust, it might look something like:

```rust
enum ViewRoleTutorial {
    /// This is the name of the role, for example it might be the name of the person who set these puzzles.
    Name,
    /// Information about this role is shown here
    Description,
    /// The puzzles that this role has created can be found here
    PuzzleList,
    /// You can click here to copy a link to this page which won't change
    Permalink,
    /// You can click here to copy a nicer shortcut link, but this might change
    Shortcut,
    /// You can click here to edit this role
    Edit,
}
```

In theory, we might want to use subtly different text depending on whether the role
is owned by the person viewing the page or not, but in practice it's probably not that
important.

What we'd like is for that enumeration to end up implementing some trait akin to:

```rust
trait TutorialItems {
    type Builder;
    fn builder() -> Self::Builder,
    fn tutorial_name() -> &'static str,
    fn first_item() -> Self,
    fn next_item(Self) -> Option<Self>,
    fn item_name(Self) -> &'static str,
    fn item_text(Self) -> &'static str,
}
```

This way, when a tutorial is created, we can look to LocalStorage and decide if we
should show it, and if so, we can find the first unviewed entry and display that item.

The easiest way to make this work is to write a derive macro for deriving TutorialItems
which ends up using the doc strings of the variants for text, etc. Deriving TutorialItems
would also create a builder which behaves approximately as:

```rust
let mut builder = ViewRoleTutorial::builder();
let name_node = use_node_ref();
builder.name(name_node);
let description_node = use_node_ref();
builder.description(description_node);
....

let tutorial = builder.build();

html! {
    <Tutorial tutorial={tutorial} />
}
```

The idea here being that each node in question which is passed into the builder enables that
builder to show that part of the tutorial, and the outcome of the `build()` function is
an instance of `TutorialData` which is approximately:

```rust
struct TutorialData {
    name: &'static str,
    nodes: Vec<(NodeRef, &'static str, &'static str)>,
}

impl TutorialData {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            nodes: vec![],
        }
    }

    fn add_node(&mut self, node: NodeRef, name: &'static str, text: &'static str) {
        self.nodes.push((node, name, text));
    }
}
```

Where the builder methods end up smelling like:

```rust
impl Builder {
    fn puzzle_links(&mut self, node: NodeRef) -> &mut Self {
        self.data.add_node(node, Self::PuzzleLinks.name(), Self::PuzzleLinks.text())
    }
}
```

Of course, if we're always using the builder to construct a tutorialdata object then there's
actually little to no point in having a TutorialItems trait, instead this could entirely
be the builder, which might actually be a better bet. In fact at that point we don't actually
need the enum to exist at all, we could construct the builder as follows:

```rust
tutorial! {
    ViewPuzzleTutorial,
    (name, "This is the name of the role, for example it might be the name of the person who set these puzzles."),
    (description, "Information about this role is shown here"),
    (puzzle_list, "The puzzles that this role has created can be found here"),
    (permalink, "You can click here to copy a link to this page which won't change"),
    (shortcutlink, "You can click here to copy a nicer shortcut link, but this might change"),
    (edit, "You can click here to edit this role"),
};
```

That would produce as output something along the lines of:

```rust
struct ViewPuzzleTutorial {
    data: TutorialData,
}

impl ViewPuzzleTutorial {
    fn builder() -> Self {
        Self {
            data: TutorialData::new("ViewPuzzleTutorial"),
        }
    }

    fn name(&mut self, node: NodeRef) -> &mut Self {
        self.data.add_node(node, "name", "This is the name of...");
        self
    }
    // ...

    fn build(self) -> TutorialData {
        self.data
    }
}
```

That might be better since then there's no need for a derive macro, that could all be written as macro_rules
which is good.
