
Turn crate into a lib

New Tool to check, that requirement IDs change whenever the requirement changes
*   Design:
    *   Tool runs on CI and pre-push
    *   Tool obtains "blessed" list of previously reviewed state (git restore --source=release .requirement-history.json)
    *   Tool computes hash for each Requirement
    *   If ReqId not found in old list, append it
    *   If ReqId is found in old list, hashes must match. Error otherwise
    *   CI will have to obtain "blessed" list, not the one from current 
*   Flags requirements where the content was modified
*   tolerant over minor changes in case, punctuation, (spelling?)
*   Review Process to flag req as unimportant



add option to artefact that requirements form this art must be matched with title,

Remove Delegation Requirements

Add Unittests for all DSGs
 
use "the third person singular present simple" verb forms in all requirements "CLI sets the process return value"
Talk about this in the readme

Add DSGs and IMPL
*   validate, that all artefacts are referenced by at least one relation
*   validate all artefacts have at least one requirement
*   validate all relations point to artefacts that exist
*   Markdown Aretefacts can be multiple MD Files

Add UseCAse:
*   Augment Input
    new cli Job to augment input artifacts with Tracing inormation / links.
    *   Each Requirement gets an added section with Links to covering/covered
        Requirements and optionally its place in the TMX.
    *   That information is ignored by parsers
    *   How to make this feel right? Should this be an external tool that uses json?
        => Cleaner, but less useable

Performance Optimizations
*   find large workload to get a real benchmark
*   use SmallString for RequirementId an  ArtefactId
*   RC all Locations
*   Intern all the Strings  (or leak them. As long as there is only the CLI, this will not hurt any)
