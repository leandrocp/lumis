# Corpus licensing decision

The stress corpus does not copy source text from any Hex package.

The inspected files include bundled JavaScript, generated SDK and protocol
output, compiled datasets, and source vendored from larger upstream projects.
A package-level license declaration is not enough to establish the provenance
and redistribution terms of each individual retained file. Rather than make a
claim that is too broad, every fixture is generated locally from original
patterns written for this repository.

The corpus records only these facts about the inspected files:

- package name and version;
- path inside the published package;
- SHA-256 digest; and
- measured byte, line, longest-line, and nesting dimensions.

Those values identify and reproduce the finding without containing expressive
source content. The synthetic source is part of Lumis and is covered by the
repository's license.

If an exact upstream file is ever proposed for this directory, its file-level
license and provenance must be verified independently before it is added. Do
not infer permission from this document or from the Hex package's top-level
license field.

