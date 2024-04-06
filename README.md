# Demo Generation

The JSON attached is manually crafted by checking available APIs which are generated through Diplomat.

It is important to note that, first we need to create `ICU4XCollator`, as this holds the functionality to compare two strings. The available to create this: 

1. Locale
2. Data Provider
3. ICU4X Collator Options

First two are easiest to tackle as they are trivial. Now we need to build the `ICU4XCollatorOptions`. This option can be selected by the user in the frontend. Now what remains is that selecting in-depth fields as well.

Do check out the Adapter in `main.rs`

Also, once this project is finished we could possibly think about integrating `schemars`. 