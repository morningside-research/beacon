# Morningside Beacon

Beacon is a tool for creating delightful knowledge graphs for researchers.

### Principles

- **"It's Just Text."** There are no databases, no binary formats. Your
  knowledge is always under your control and never behind any layers.

- **Blazing Fast Retrieval** What's the use in storing knowledge that you can
  never sift through? Full text search and light-speed queries mean your
  knowledge is always at your fingertips.

- **Excellent at one thing** Your favorite editor already exists. Your favorite
  version control system already exists. Beacon is an excellent document manager
  and that's all it wants to be.

- **Plays Well With others** Beacon's is extensible. Beacon wants to output your
  database to Markdown in as many ways as possible. Output the result of your
  query as a standalone Markdown document -- nice thesis you got there. Or
  perhaps link-connected documents for fast conversion to a static site. However
  you slice it, Beacon will export it.

## The Beacon Standard 0.1.0 Draft 1

Beacon is a standard. Morningside Beacon is an implementation.

### Definitions

An **index** is a directory with the text prefix `index` which also contains
text files called nodes.

The **records** of an index are stored in a directory at the same level of that
index with the text prefix `records`.

A **node** is a text file within the index. A node is a valid `yaml` file which
necessarily contains the following immutable data. The key value pairs of this
file are to be called metadata. A node may also be the same structure in the
memory of some process with all of its analogous parts.

A **user-string** is a string where the value is input by the user.

An **attribute** is a `yaml` file which has its URI in another node's
`attribute-uri`. The attribute is stored in the `records` directory.

- a `time-created` which is a valid ISO-8601 timestamp;

- a `mime-type` which is the text string containing a valid mimetype of the data
  pointed to;

- a `content-uri` which is a URI to the content in the mimetype;

- a `node-title` which is a user-string.

- a `tags` field which is a list containing user-strings;

- a `collections` field which is a list containing user-strings; and

- a `attribute-uris` field which is list of URI's to attribute-documents;

Nodes are immutable, which means each alteration will provision a new node with
the alteration made. These alterations are called **mutations**. These
conditions might permit the accumulation of unused nodes, which is undesirable.
So within the records let there be files prefixed `instruction` containing

- `instruction-type` which is one of

   - "detach"
   - and perhaps others.
  
- `content-uri`

When a node is no longer needed, a new file can be written to register its
detachment. Later a query may be made to print all detached nodes to a file,
which can then be used to delete them. Beacon will never directly delete any
nodes.

TODO: 
- Queries
- Caching
- Virtual file system

### Morningside Features

Morningside Beacon promotes

- Jinja templates for parametric markup
- Fetching and Caching of web-hosted resources
