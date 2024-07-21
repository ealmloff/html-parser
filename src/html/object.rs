use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum ObjectAttributes {
    Accesskey(String),
    Archive(String),
    AriaActivedescendant(String),
    AriaAtomic(crate::BValues),
    AriaAutocomplete(crate::AutocompleteValues),
    AriaBusy(crate::BValues),
    AriaChecked(crate::TristateValues),
    AriaColcount(String),
    AriaColindex(String),
    AriaColspan(String),
    AriaControls(String),
    AriaCurrent(crate::CurrentValues),
    AriaDescribedby(String),
    AriaDetails(String),
    AriaDisabled(crate::BValues),
    AriaDropeffect(crate::DropeffectValues),
    AriaErrormessage(String),
    AriaExpanded(crate::UValues),
    AriaFlowto(String),
    AriaGrabbed(crate::UValues),
    AriaHaspopup(crate::HaspopupValues),
    AriaHidden(crate::BValues),
    AriaInvalid(crate::InvalidValues),
    AriaKeyshortcuts(String),
    AriaLabel(String),
    AriaLabelledby(String),
    AriaLevel(String),
    AriaLive(crate::LiveValues),
    AriaModal(crate::BValues),
    AriaMultiline(crate::BValues),
    AriaMultiselectable(crate::BValues),
    AriaOrientation(crate::OrientationValues),
    AriaOwns(String),
    AriaPlaceholder(String),
    AriaPosinset(String),
    AriaPressed(crate::TristateValues),
    AriaReadonly(crate::BValues),
    AriaRelevant(crate::RelevantValues),
    AriaRequired(crate::BValues),
    AriaRoledescription(String),
    AriaRowcount(String),
    AriaRowindex(String),
    AriaRowspan(String),
    AriaSelected(crate::UValues),
    AriaSetsize(String),
    AriaSort(crate::SortValues),
    AriaValuemax(String),
    AriaValuemin(String),
    AriaValuenow(String),
    AriaValuetext(String),
    Autocapitalize(String),
    Border(String),
    Class(String),
    Classid(String),
    Codebase(String),
    Codetype(String),
    Contenteditable(String),
    Contextmenu(String),
    Data(String),
    Declare(String),
    Dir(crate::DValues),
    Draggable(crate::BValues),
    Dropzone(String),
    Exportparts(String),
    Form(String),
    Height(String),
    Hidden(String),
    Id(String),
    Inputmode(String),
    Is(String),
    Itemid(String),
    Itemprop(String),
    Itemref(String),
    Itemscope(String),
    Itemtype(String),
    Lang(String),
    Name(String),
    Onabort(String),
    Onblur(String),
    Oncanplay(String),
    Oncanplaythrough(String),
    Onchange(String),
    Onclick(String),
    Oncontextmenu(String),
    Ondblclick(String),
    Ondrag(String),
    Ondragend(String),
    Ondragenter(String),
    Ondragleave(String),
    Ondragover(String),
    Ondragstart(String),
    Ondrop(String),
    Ondurationchange(String),
    Onemptied(String),
    Onended(String),
    Onerror(String),
    Onfocus(String),
    Onformchange(String),
    Onforminput(String),
    Oninput(String),
    Oninvalid(String),
    Onkeydown(String),
    Onkeypress(String),
    Onkeyup(String),
    Onload(String),
    Onloadeddata(String),
    Onloadedmetadata(String),
    Onloadstart(String),
    Onmousedown(String),
    Onmouseenter(String),
    Onmouseleave(String),
    Onmousemove(String),
    Onmouseout(String),
    Onmouseover(String),
    Onmouseup(String),
    Onmousewheel(String),
    Onpause(String),
    Onplay(String),
    Onplaying(String),
    Onpointercancel(String),
    Onpointerdown(String),
    Onpointerenter(String),
    Onpointerleave(String),
    Onpointerlockchange(String),
    Onpointerlockerror(String),
    Onpointermove(String),
    Onpointerout(String),
    Onpointerover(String),
    Onpointerup(String),
    Onprogress(String),
    Onratechange(String),
    Onreadystatechange(String),
    Onreset(String),
    Onresize(String),
    Onscroll(String),
    Onseeked(String),
    Onseeking(String),
    Onselect(String),
    Onshow(String),
    Onstalled(String),
    Onsubmit(String),
    Onsuspend(String),
    Ontimeupdate(String),
    Onvolumechange(String),
    Onwaiting(String),
    Part(String),
    Role(crate::RolesValues),
    Slot(String),
    Spellcheck(crate::BValues),
    Standby(String),
    Style(String),
    Tabindex(String),
    Title(String),
    Translate(crate::YValues),
    Type(String),
    Typemustmatch(String),
    Usemap(String),
    Width(String),
}
impl kalosm_sample::Parse for ObjectAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        LiteralParser::new("\"accesskey\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Accesskey(value))
        .or(
        LiteralParser::new("\"archive\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Archive(value))
        )
        .or(
        LiteralParser::new("\"aria-activedescendant\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaActivedescendant(value))
        )
        .or(
        LiteralParser::new("\"aria-atomic\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::AriaAtomic(value))
        )
        .or(
        LiteralParser::new("\"aria-autocomplete\"=")
            .ignore_output_then(crate::AutocompleteValues::new_parser())
            .map_output(|value| Self::AriaAutocomplete(value))
        )
        .or(
        LiteralParser::new("\"aria-busy\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::AriaBusy(value))
        )
        .or(
        LiteralParser::new("\"aria-checked\"=")
            .ignore_output_then(crate::TristateValues::new_parser())
            .map_output(|value| Self::AriaChecked(value))
        )
        .or(
        LiteralParser::new("\"aria-colcount\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaColcount(value))
        )
        .or(
        LiteralParser::new("\"aria-colindex\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaColindex(value))
        )
        .or(
        LiteralParser::new("\"aria-colspan\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaColspan(value))
        )
        .or(
        LiteralParser::new("\"aria-controls\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaControls(value))
        )
        .or(
        LiteralParser::new("\"aria-current\"=")
            .ignore_output_then(crate::CurrentValues::new_parser())
            .map_output(|value| Self::AriaCurrent(value))
        )
        .or(
        LiteralParser::new("\"aria-describedby\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaDescribedby(value))
        )
        .or(
        LiteralParser::new("\"aria-details\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaDetails(value))
        )
        .or(
        LiteralParser::new("\"aria-disabled\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::AriaDisabled(value))
        )
        .or(
        LiteralParser::new("\"aria-dropeffect\"=")
            .ignore_output_then(crate::DropeffectValues::new_parser())
            .map_output(|value| Self::AriaDropeffect(value))
        )
        .or(
        LiteralParser::new("\"aria-errormessage\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaErrormessage(value))
        )
        .or(
        LiteralParser::new("\"aria-expanded\"=")
            .ignore_output_then(crate::UValues::new_parser())
            .map_output(|value| Self::AriaExpanded(value))
        )
        .or(
        LiteralParser::new("\"aria-flowto\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaFlowto(value))
        )
        .or(
        LiteralParser::new("\"aria-grabbed\"=")
            .ignore_output_then(crate::UValues::new_parser())
            .map_output(|value| Self::AriaGrabbed(value))
        )
        .or(
        LiteralParser::new("\"aria-haspopup\"=")
            .ignore_output_then(crate::HaspopupValues::new_parser())
            .map_output(|value| Self::AriaHaspopup(value))
        )
        .or(
        LiteralParser::new("\"aria-hidden\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::AriaHidden(value))
        )
        .or(
        LiteralParser::new("\"aria-invalid\"=")
            .ignore_output_then(crate::InvalidValues::new_parser())
            .map_output(|value| Self::AriaInvalid(value))
        )
        .or(
        LiteralParser::new("\"aria-keyshortcuts\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaKeyshortcuts(value))
        )
        .or(
        LiteralParser::new("\"aria-label\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaLabel(value))
        )
        .or(
        LiteralParser::new("\"aria-labelledby\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaLabelledby(value))
        )
        .or(
        LiteralParser::new("\"aria-level\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaLevel(value))
        )
        .or(
        LiteralParser::new("\"aria-live\"=")
            .ignore_output_then(crate::LiveValues::new_parser())
            .map_output(|value| Self::AriaLive(value))
        )
        .or(
        LiteralParser::new("\"aria-modal\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::AriaModal(value))
        )
        .or(
        LiteralParser::new("\"aria-multiline\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::AriaMultiline(value))
        )
        .or(
        LiteralParser::new("\"aria-multiselectable\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::AriaMultiselectable(value))
        )
        .or(
        LiteralParser::new("\"aria-orientation\"=")
            .ignore_output_then(crate::OrientationValues::new_parser())
            .map_output(|value| Self::AriaOrientation(value))
        )
        .or(
        LiteralParser::new("\"aria-owns\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaOwns(value))
        )
        .or(
        LiteralParser::new("\"aria-placeholder\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaPlaceholder(value))
        )
        .or(
        LiteralParser::new("\"aria-posinset\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaPosinset(value))
        )
        .or(
        LiteralParser::new("\"aria-pressed\"=")
            .ignore_output_then(crate::TristateValues::new_parser())
            .map_output(|value| Self::AriaPressed(value))
        )
        .or(
        LiteralParser::new("\"aria-readonly\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::AriaReadonly(value))
        )
        .or(
        LiteralParser::new("\"aria-relevant\"=")
            .ignore_output_then(crate::RelevantValues::new_parser())
            .map_output(|value| Self::AriaRelevant(value))
        )
        .or(
        LiteralParser::new("\"aria-required\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::AriaRequired(value))
        )
        .or(
        LiteralParser::new("\"aria-roledescription\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaRoledescription(value))
        )
        .or(
        LiteralParser::new("\"aria-rowcount\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaRowcount(value))
        )
        .or(
        LiteralParser::new("\"aria-rowindex\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaRowindex(value))
        )
        .or(
        LiteralParser::new("\"aria-rowspan\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaRowspan(value))
        )
        .or(
        LiteralParser::new("\"aria-selected\"=")
            .ignore_output_then(crate::UValues::new_parser())
            .map_output(|value| Self::AriaSelected(value))
        )
        .or(
        LiteralParser::new("\"aria-setsize\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaSetsize(value))
        )
        .or(
        LiteralParser::new("\"aria-sort\"=")
            .ignore_output_then(crate::SortValues::new_parser())
            .map_output(|value| Self::AriaSort(value))
        )
        .or(
        LiteralParser::new("\"aria-valuemax\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaValuemax(value))
        )
        .or(
        LiteralParser::new("\"aria-valuemin\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaValuemin(value))
        )
        .or(
        LiteralParser::new("\"aria-valuenow\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaValuenow(value))
        )
        .or(
        LiteralParser::new("\"aria-valuetext\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::AriaValuetext(value))
        )
        .or(
        LiteralParser::new("\"autocapitalize\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Autocapitalize(value))
        )
        .or(
        LiteralParser::new("\"border\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Border(value))
        )
        .or(
        LiteralParser::new("\"class\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Class(value))
        )
        .or(
        LiteralParser::new("\"classid\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Classid(value))
        )
        .or(
        LiteralParser::new("\"codebase\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Codebase(value))
        )
        .or(
        LiteralParser::new("\"codetype\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Codetype(value))
        )
        .or(
        LiteralParser::new("\"contenteditable\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Contenteditable(value))
        )
        .or(
        LiteralParser::new("\"contextmenu\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Contextmenu(value))
        )
        .or(
        LiteralParser::new("\"data\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Data(value))
        )
        .or(
        LiteralParser::new("\"declare\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Declare(value))
        )
        .or(
        LiteralParser::new("\"dir\"=")
            .ignore_output_then(crate::DValues::new_parser())
            .map_output(|value| Self::Dir(value))
        )
        .or(
        LiteralParser::new("\"draggable\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::Draggable(value))
        )
        .or(
        LiteralParser::new("\"dropzone\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Dropzone(value))
        )
        .or(
        LiteralParser::new("\"exportparts\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Exportparts(value))
        )
        .or(
        LiteralParser::new("\"form\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Form(value))
        )
        .or(
        LiteralParser::new("\"height\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Height(value))
        )
        .or(
        LiteralParser::new("\"hidden\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Hidden(value))
        )
        .or(
        LiteralParser::new("\"id\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Id(value))
        )
        .or(
        LiteralParser::new("\"inputmode\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Inputmode(value))
        )
        .or(
        LiteralParser::new("\"is\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Is(value))
        )
        .or(
        LiteralParser::new("\"itemid\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Itemid(value))
        )
        .or(
        LiteralParser::new("\"itemprop\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Itemprop(value))
        )
        .or(
        LiteralParser::new("\"itemref\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Itemref(value))
        )
        .or(
        LiteralParser::new("\"itemscope\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Itemscope(value))
        )
        .or(
        LiteralParser::new("\"itemtype\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Itemtype(value))
        )
        .or(
        LiteralParser::new("\"lang\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Lang(value))
        )
        .or(
        LiteralParser::new("\"name\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Name(value))
        )
        .or(
        LiteralParser::new("\"onabort\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onabort(value))
        )
        .or(
        LiteralParser::new("\"onblur\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onblur(value))
        )
        .or(
        LiteralParser::new("\"oncanplay\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Oncanplay(value))
        )
        .or(
        LiteralParser::new("\"oncanplaythrough\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Oncanplaythrough(value))
        )
        .or(
        LiteralParser::new("\"onchange\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onchange(value))
        )
        .or(
        LiteralParser::new("\"onclick\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onclick(value))
        )
        .or(
        LiteralParser::new("\"oncontextmenu\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Oncontextmenu(value))
        )
        .or(
        LiteralParser::new("\"ondblclick\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ondblclick(value))
        )
        .or(
        LiteralParser::new("\"ondrag\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ondrag(value))
        )
        .or(
        LiteralParser::new("\"ondragend\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ondragend(value))
        )
        .or(
        LiteralParser::new("\"ondragenter\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ondragenter(value))
        )
        .or(
        LiteralParser::new("\"ondragleave\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ondragleave(value))
        )
        .or(
        LiteralParser::new("\"ondragover\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ondragover(value))
        )
        .or(
        LiteralParser::new("\"ondragstart\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ondragstart(value))
        )
        .or(
        LiteralParser::new("\"ondrop\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ondrop(value))
        )
        .or(
        LiteralParser::new("\"ondurationchange\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ondurationchange(value))
        )
        .or(
        LiteralParser::new("\"onemptied\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onemptied(value))
        )
        .or(
        LiteralParser::new("\"onended\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onended(value))
        )
        .or(
        LiteralParser::new("\"onerror\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onerror(value))
        )
        .or(
        LiteralParser::new("\"onfocus\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onfocus(value))
        )
        .or(
        LiteralParser::new("\"onformchange\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onformchange(value))
        )
        .or(
        LiteralParser::new("\"onforminput\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onforminput(value))
        )
        .or(
        LiteralParser::new("\"oninput\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Oninput(value))
        )
        .or(
        LiteralParser::new("\"oninvalid\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Oninvalid(value))
        )
        .or(
        LiteralParser::new("\"onkeydown\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onkeydown(value))
        )
        .or(
        LiteralParser::new("\"onkeypress\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onkeypress(value))
        )
        .or(
        LiteralParser::new("\"onkeyup\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onkeyup(value))
        )
        .or(
        LiteralParser::new("\"onload\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onload(value))
        )
        .or(
        LiteralParser::new("\"onloadeddata\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onloadeddata(value))
        )
        .or(
        LiteralParser::new("\"onloadedmetadata\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onloadedmetadata(value))
        )
        .or(
        LiteralParser::new("\"onloadstart\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onloadstart(value))
        )
        .or(
        LiteralParser::new("\"onmousedown\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onmousedown(value))
        )
        .or(
        LiteralParser::new("\"onmouseenter\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onmouseenter(value))
        )
        .or(
        LiteralParser::new("\"onmouseleave\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onmouseleave(value))
        )
        .or(
        LiteralParser::new("\"onmousemove\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onmousemove(value))
        )
        .or(
        LiteralParser::new("\"onmouseout\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onmouseout(value))
        )
        .or(
        LiteralParser::new("\"onmouseover\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onmouseover(value))
        )
        .or(
        LiteralParser::new("\"onmouseup\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onmouseup(value))
        )
        .or(
        LiteralParser::new("\"onmousewheel\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onmousewheel(value))
        )
        .or(
        LiteralParser::new("\"onpause\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpause(value))
        )
        .or(
        LiteralParser::new("\"onplay\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onplay(value))
        )
        .or(
        LiteralParser::new("\"onplaying\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onplaying(value))
        )
        .or(
        LiteralParser::new("\"onpointercancel\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointercancel(value))
        )
        .or(
        LiteralParser::new("\"onpointerdown\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointerdown(value))
        )
        .or(
        LiteralParser::new("\"onpointerenter\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointerenter(value))
        )
        .or(
        LiteralParser::new("\"onpointerleave\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointerleave(value))
        )
        .or(
        LiteralParser::new("\"onpointerlockchange\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointerlockchange(value))
        )
        .or(
        LiteralParser::new("\"onpointerlockerror\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointerlockerror(value))
        )
        .or(
        LiteralParser::new("\"onpointermove\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointermove(value))
        )
        .or(
        LiteralParser::new("\"onpointerout\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointerout(value))
        )
        .or(
        LiteralParser::new("\"onpointerover\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointerover(value))
        )
        .or(
        LiteralParser::new("\"onpointerup\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onpointerup(value))
        )
        .or(
        LiteralParser::new("\"onprogress\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onprogress(value))
        )
        .or(
        LiteralParser::new("\"onratechange\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onratechange(value))
        )
        .or(
        LiteralParser::new("\"onreadystatechange\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onreadystatechange(value))
        )
        .or(
        LiteralParser::new("\"onreset\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onreset(value))
        )
        .or(
        LiteralParser::new("\"onresize\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onresize(value))
        )
        .or(
        LiteralParser::new("\"onscroll\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onscroll(value))
        )
        .or(
        LiteralParser::new("\"onseeked\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onseeked(value))
        )
        .or(
        LiteralParser::new("\"onseeking\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onseeking(value))
        )
        .or(
        LiteralParser::new("\"onselect\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onselect(value))
        )
        .or(
        LiteralParser::new("\"onshow\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onshow(value))
        )
        .or(
        LiteralParser::new("\"onstalled\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onstalled(value))
        )
        .or(
        LiteralParser::new("\"onsubmit\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onsubmit(value))
        )
        .or(
        LiteralParser::new("\"onsuspend\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onsuspend(value))
        )
        .or(
        LiteralParser::new("\"ontimeupdate\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Ontimeupdate(value))
        )
        .or(
        LiteralParser::new("\"onvolumechange\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onvolumechange(value))
        )
        .or(
        LiteralParser::new("\"onwaiting\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Onwaiting(value))
        )
        .or(
        LiteralParser::new("\"part\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Part(value))
        )
        .or(
        LiteralParser::new("\"role\"=")
            .ignore_output_then(crate::RolesValues::new_parser())
            .map_output(|value| Self::Role(value))
        )
        .or(
        LiteralParser::new("\"slot\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Slot(value))
        )
        .or(
        LiteralParser::new("\"spellcheck\"=")
            .ignore_output_then(crate::BValues::new_parser())
            .map_output(|value| Self::Spellcheck(value))
        )
        .or(
        LiteralParser::new("\"standby\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Standby(value))
        )
        .or(
        LiteralParser::new("\"style\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Style(value))
        )
        .or(
        LiteralParser::new("\"tabindex\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Tabindex(value))
        )
        .or(
        LiteralParser::new("\"title\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Title(value))
        )
        .or(
        LiteralParser::new("\"translate\"=")
            .ignore_output_then(crate::YValues::new_parser())
            .map_output(|value| Self::Translate(value))
        )
        .or(
        LiteralParser::new("\"type\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Type(value))
        )
        .or(
        LiteralParser::new("\"typemustmatch\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Typemustmatch(value))
        )
        .or(
        LiteralParser::new("\"usemap\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Usemap(value))
        )
        .or(
        LiteralParser::new("\"width\"=")
            .ignore_output_then(String::new_parser())
            .map_output(|value| Self::Width(value))
        )
    }
}
#[derive(Debug, Clone)]
pub struct Object{
    attributes: Vec<ObjectAttributes>,
    body: Vec<crate::Element>,
}
impl kalosm_sample::Parse for Object {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        LiteralParser::new("<object")
            .ignore_output_then(
                LiteralParser::new(" ")
                    .ignore_output_then(ObjectAttributes::new_parser())
                    .repeat(0..=10000)
            )
            .then_literal(">")
            .then(crate::Element::new_parser().boxed().repeat(0..=10000))
            .then_literal("</object>")
            .map_output(|(attributes, body)| Object { attributes, body })
    }
}
