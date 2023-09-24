# PlainText Styles (PTS)

## What is PlainText Styles

PlainText Styles is an alter alternative to Markdown, designed to have more human readable syntax

## The Basics

### Firstly, a **BRAND** new feature: _metadata!_!

Metadata is defined like this:

```js
¦key: value¦
```

or like this:

```js
¦¦¦
  key: value
  other_key: value
¦¦¦
```

The most common metadata is `¦version: 1.0¦` or some other version, without it the compiler will interpret it as the latest version

### Additionally, the _italic_ syntax makes more sense by using /

`PTS`

```
//Italic//
```

`MD`

```
_Italic_
*Italic*
```

### There isn't just a bold syntax, there is font weight

In markdown you use \*\*asterisk\*\* syntax for **bold**

but in PTS you specify the font weight:

```
*Light* (font weight 100)
**Light** (font weight 200)
```

### Use [] for repeating

If you want 50 of the letter a you could type:

```
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
```

or you could type:

```
a[50]
[a][50]
```
