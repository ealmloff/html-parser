use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum EmbedAttributesName {
    #[parse(rename = " accesskey=")]
    Accesskey,
    #[parse(rename = " aria-activedescendant=")]
    AriaActivedescendant,
    #[parse(rename = " aria-atomic=")]
    AriaAtomic,
    #[parse(rename = " aria-autocomplete=")]
    AriaAutocomplete,
    #[parse(rename = " aria-busy=")]
    AriaBusy,
    #[parse(rename = " aria-checked=")]
    AriaChecked,
    #[parse(rename = " aria-colcount=")]
    AriaColcount,
    #[parse(rename = " aria-colindex=")]
    AriaColindex,
    #[parse(rename = " aria-colspan=")]
    AriaColspan,
    #[parse(rename = " aria-controls=")]
    AriaControls,
    #[parse(rename = " aria-current=")]
    AriaCurrent,
    #[parse(rename = " aria-describedby=")]
    AriaDescribedby,
    #[parse(rename = " aria-details=")]
    AriaDetails,
    #[parse(rename = " aria-disabled=")]
    AriaDisabled,
    #[parse(rename = " aria-dropeffect=")]
    AriaDropeffect,
    #[parse(rename = " aria-errormessage=")]
    AriaErrormessage,
    #[parse(rename = " aria-expanded=")]
    AriaExpanded,
    #[parse(rename = " aria-flowto=")]
    AriaFlowto,
    #[parse(rename = " aria-grabbed=")]
    AriaGrabbed,
    #[parse(rename = " aria-haspopup=")]
    AriaHaspopup,
    #[parse(rename = " aria-hidden=")]
    AriaHidden,
    #[parse(rename = " aria-invalid=")]
    AriaInvalid,
    #[parse(rename = " aria-keyshortcuts=")]
    AriaKeyshortcuts,
    #[parse(rename = " aria-label=")]
    AriaLabel,
    #[parse(rename = " aria-labelledby=")]
    AriaLabelledby,
    #[parse(rename = " aria-level=")]
    AriaLevel,
    #[parse(rename = " aria-live=")]
    AriaLive,
    #[parse(rename = " aria-modal=")]
    AriaModal,
    #[parse(rename = " aria-multiline=")]
    AriaMultiline,
    #[parse(rename = " aria-multiselectable=")]
    AriaMultiselectable,
    #[parse(rename = " aria-orientation=")]
    AriaOrientation,
    #[parse(rename = " aria-owns=")]
    AriaOwns,
    #[parse(rename = " aria-placeholder=")]
    AriaPlaceholder,
    #[parse(rename = " aria-posinset=")]
    AriaPosinset,
    #[parse(rename = " aria-pressed=")]
    AriaPressed,
    #[parse(rename = " aria-readonly=")]
    AriaReadonly,
    #[parse(rename = " aria-relevant=")]
    AriaRelevant,
    #[parse(rename = " aria-required=")]
    AriaRequired,
    #[parse(rename = " aria-roledescription=")]
    AriaRoledescription,
    #[parse(rename = " aria-rowcount=")]
    AriaRowcount,
    #[parse(rename = " aria-rowindex=")]
    AriaRowindex,
    #[parse(rename = " aria-rowspan=")]
    AriaRowspan,
    #[parse(rename = " aria-selected=")]
    AriaSelected,
    #[parse(rename = " aria-setsize=")]
    AriaSetsize,
    #[parse(rename = " aria-sort=")]
    AriaSort,
    #[parse(rename = " aria-valuemax=")]
    AriaValuemax,
    #[parse(rename = " aria-valuemin=")]
    AriaValuemin,
    #[parse(rename = " aria-valuenow=")]
    AriaValuenow,
    #[parse(rename = " aria-valuetext=")]
    AriaValuetext,
    #[parse(rename = " autocapitalize=")]
    Autocapitalize,
    #[parse(rename = " class=")]
    Class,
    #[parse(rename = " contenteditable=")]
    Contenteditable,
    #[parse(rename = " contextmenu=")]
    Contextmenu,
    #[parse(rename = " dir=")]
    Dir,
    #[parse(rename = " draggable=")]
    Draggable,
    #[parse(rename = " dropzone=")]
    Dropzone,
    #[parse(rename = " exportparts=")]
    Exportparts,
    #[parse(rename = " height=")]
    Height,
    #[parse(rename = " hidden=")]
    Hidden,
    #[parse(rename = " id=")]
    Id,
    #[parse(rename = " inputmode=")]
    Inputmode,
    #[parse(rename = " is=")]
    Is,
    #[parse(rename = " itemid=")]
    Itemid,
    #[parse(rename = " itemprop=")]
    Itemprop,
    #[parse(rename = " itemref=")]
    Itemref,
    #[parse(rename = " itemscope=")]
    Itemscope,
    #[parse(rename = " itemtype=")]
    Itemtype,
    #[parse(rename = " lang=")]
    Lang,
    #[parse(rename = " onabort=")]
    Onabort,
    #[parse(rename = " onblur=")]
    Onblur,
    #[parse(rename = " oncanplay=")]
    Oncanplay,
    #[parse(rename = " oncanplaythrough=")]
    Oncanplaythrough,
    #[parse(rename = " onchange=")]
    Onchange,
    #[parse(rename = " onclick=")]
    Onclick,
    #[parse(rename = " oncontextmenu=")]
    Oncontextmenu,
    #[parse(rename = " ondblclick=")]
    Ondblclick,
    #[parse(rename = " ondrag=")]
    Ondrag,
    #[parse(rename = " ondragend=")]
    Ondragend,
    #[parse(rename = " ondragenter=")]
    Ondragenter,
    #[parse(rename = " ondragleave=")]
    Ondragleave,
    #[parse(rename = " ondragover=")]
    Ondragover,
    #[parse(rename = " ondragstart=")]
    Ondragstart,
    #[parse(rename = " ondrop=")]
    Ondrop,
    #[parse(rename = " ondurationchange=")]
    Ondurationchange,
    #[parse(rename = " onemptied=")]
    Onemptied,
    #[parse(rename = " onended=")]
    Onended,
    #[parse(rename = " onerror=")]
    Onerror,
    #[parse(rename = " onfocus=")]
    Onfocus,
    #[parse(rename = " onformchange=")]
    Onformchange,
    #[parse(rename = " onforminput=")]
    Onforminput,
    #[parse(rename = " oninput=")]
    Oninput,
    #[parse(rename = " oninvalid=")]
    Oninvalid,
    #[parse(rename = " onkeydown=")]
    Onkeydown,
    #[parse(rename = " onkeypress=")]
    Onkeypress,
    #[parse(rename = " onkeyup=")]
    Onkeyup,
    #[parse(rename = " onload=")]
    Onload,
    #[parse(rename = " onloadeddata=")]
    Onloadeddata,
    #[parse(rename = " onloadedmetadata=")]
    Onloadedmetadata,
    #[parse(rename = " onloadstart=")]
    Onloadstart,
    #[parse(rename = " onmousedown=")]
    Onmousedown,
    #[parse(rename = " onmouseenter=")]
    Onmouseenter,
    #[parse(rename = " onmouseleave=")]
    Onmouseleave,
    #[parse(rename = " onmousemove=")]
    Onmousemove,
    #[parse(rename = " onmouseout=")]
    Onmouseout,
    #[parse(rename = " onmouseover=")]
    Onmouseover,
    #[parse(rename = " onmouseup=")]
    Onmouseup,
    #[parse(rename = " onmousewheel=")]
    Onmousewheel,
    #[parse(rename = " onpause=")]
    Onpause,
    #[parse(rename = " onplay=")]
    Onplay,
    #[parse(rename = " onplaying=")]
    Onplaying,
    #[parse(rename = " onpointercancel=")]
    Onpointercancel,
    #[parse(rename = " onpointerdown=")]
    Onpointerdown,
    #[parse(rename = " onpointerenter=")]
    Onpointerenter,
    #[parse(rename = " onpointerleave=")]
    Onpointerleave,
    #[parse(rename = " onpointerlockchange=")]
    Onpointerlockchange,
    #[parse(rename = " onpointerlockerror=")]
    Onpointerlockerror,
    #[parse(rename = " onpointermove=")]
    Onpointermove,
    #[parse(rename = " onpointerout=")]
    Onpointerout,
    #[parse(rename = " onpointerover=")]
    Onpointerover,
    #[parse(rename = " onpointerup=")]
    Onpointerup,
    #[parse(rename = " onprogress=")]
    Onprogress,
    #[parse(rename = " onratechange=")]
    Onratechange,
    #[parse(rename = " onreadystatechange=")]
    Onreadystatechange,
    #[parse(rename = " onreset=")]
    Onreset,
    #[parse(rename = " onresize=")]
    Onresize,
    #[parse(rename = " onscroll=")]
    Onscroll,
    #[parse(rename = " onseeked=")]
    Onseeked,
    #[parse(rename = " onseeking=")]
    Onseeking,
    #[parse(rename = " onselect=")]
    Onselect,
    #[parse(rename = " onshow=")]
    Onshow,
    #[parse(rename = " onstalled=")]
    Onstalled,
    #[parse(rename = " onsubmit=")]
    Onsubmit,
    #[parse(rename = " onsuspend=")]
    Onsuspend,
    #[parse(rename = " ontimeupdate=")]
    Ontimeupdate,
    #[parse(rename = " onvolumechange=")]
    Onvolumechange,
    #[parse(rename = " onwaiting=")]
    Onwaiting,
    #[parse(rename = " part=")]
    Part,
    #[parse(rename = " role=")]
    Role,
    #[parse(rename = " slot=")]
    Slot,
    #[parse(rename = " spellcheck=")]
    Spellcheck,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " style=")]
    Style,
    #[parse(rename = " tabindex=")]
    Tabindex,
    #[parse(rename = " title=")]
    Title,
    #[parse(rename = " translate=")]
    Translate,
    #[parse(rename = " type=")]
    Type,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum EmbedAttributes {
    Accesskey(String),
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
    Class(String),
    Contenteditable(String),
    Contextmenu(String),
    Dir(crate::DValues),
    Draggable(crate::BValues),
    Dropzone(String),
    Exportparts(String),
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
    Src(String),
    Style(String),
    Tabindex(String),
    Title(String),
    Translate(crate::YValues),
    Type(String),
    Width(String),
}
impl kalosm_sample::Parse for EmbedAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        EmbedAttributesName::new_parser()
            .then_lazy(|name| match name {
                EmbedAttributesName::Accesskey => {
                    String::new_parser().map_output(Self::Accesskey).boxed()
                }
                EmbedAttributesName::AriaActivedescendant => String::new_parser()
                    .map_output(Self::AriaActivedescendant)
                    .boxed(),
                EmbedAttributesName::AriaAtomic => crate::BValues::new_parser()
                    .map_output(Self::AriaAtomic)
                    .boxed(),
                EmbedAttributesName::AriaAutocomplete => crate::AutocompleteValues::new_parser()
                    .map_output(Self::AriaAutocomplete)
                    .boxed(),
                EmbedAttributesName::AriaBusy => crate::BValues::new_parser()
                    .map_output(Self::AriaBusy)
                    .boxed(),
                EmbedAttributesName::AriaChecked => crate::TristateValues::new_parser()
                    .map_output(Self::AriaChecked)
                    .boxed(),
                EmbedAttributesName::AriaColcount => {
                    String::new_parser().map_output(Self::AriaColcount).boxed()
                }
                EmbedAttributesName::AriaColindex => {
                    String::new_parser().map_output(Self::AriaColindex).boxed()
                }
                EmbedAttributesName::AriaColspan => {
                    String::new_parser().map_output(Self::AriaColspan).boxed()
                }
                EmbedAttributesName::AriaControls => {
                    String::new_parser().map_output(Self::AriaControls).boxed()
                }
                EmbedAttributesName::AriaCurrent => crate::CurrentValues::new_parser()
                    .map_output(Self::AriaCurrent)
                    .boxed(),
                EmbedAttributesName::AriaDescribedby => String::new_parser()
                    .map_output(Self::AriaDescribedby)
                    .boxed(),
                EmbedAttributesName::AriaDetails => {
                    String::new_parser().map_output(Self::AriaDetails).boxed()
                }
                EmbedAttributesName::AriaDisabled => crate::BValues::new_parser()
                    .map_output(Self::AriaDisabled)
                    .boxed(),
                EmbedAttributesName::AriaDropeffect => crate::DropeffectValues::new_parser()
                    .map_output(Self::AriaDropeffect)
                    .boxed(),
                EmbedAttributesName::AriaErrormessage => String::new_parser()
                    .map_output(Self::AriaErrormessage)
                    .boxed(),
                EmbedAttributesName::AriaExpanded => crate::UValues::new_parser()
                    .map_output(Self::AriaExpanded)
                    .boxed(),
                EmbedAttributesName::AriaFlowto => {
                    String::new_parser().map_output(Self::AriaFlowto).boxed()
                }
                EmbedAttributesName::AriaGrabbed => crate::UValues::new_parser()
                    .map_output(Self::AriaGrabbed)
                    .boxed(),
                EmbedAttributesName::AriaHaspopup => crate::HaspopupValues::new_parser()
                    .map_output(Self::AriaHaspopup)
                    .boxed(),
                EmbedAttributesName::AriaHidden => crate::BValues::new_parser()
                    .map_output(Self::AriaHidden)
                    .boxed(),
                EmbedAttributesName::AriaInvalid => crate::InvalidValues::new_parser()
                    .map_output(Self::AriaInvalid)
                    .boxed(),
                EmbedAttributesName::AriaKeyshortcuts => String::new_parser()
                    .map_output(Self::AriaKeyshortcuts)
                    .boxed(),
                EmbedAttributesName::AriaLabel => {
                    String::new_parser().map_output(Self::AriaLabel).boxed()
                }
                EmbedAttributesName::AriaLabelledby => String::new_parser()
                    .map_output(Self::AriaLabelledby)
                    .boxed(),
                EmbedAttributesName::AriaLevel => {
                    String::new_parser().map_output(Self::AriaLevel).boxed()
                }
                EmbedAttributesName::AriaLive => crate::LiveValues::new_parser()
                    .map_output(Self::AriaLive)
                    .boxed(),
                EmbedAttributesName::AriaModal => crate::BValues::new_parser()
                    .map_output(Self::AriaModal)
                    .boxed(),
                EmbedAttributesName::AriaMultiline => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiline)
                    .boxed(),
                EmbedAttributesName::AriaMultiselectable => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiselectable)
                    .boxed(),
                EmbedAttributesName::AriaOrientation => crate::OrientationValues::new_parser()
                    .map_output(Self::AriaOrientation)
                    .boxed(),
                EmbedAttributesName::AriaOwns => {
                    String::new_parser().map_output(Self::AriaOwns).boxed()
                }
                EmbedAttributesName::AriaPlaceholder => String::new_parser()
                    .map_output(Self::AriaPlaceholder)
                    .boxed(),
                EmbedAttributesName::AriaPosinset => {
                    String::new_parser().map_output(Self::AriaPosinset).boxed()
                }
                EmbedAttributesName::AriaPressed => crate::TristateValues::new_parser()
                    .map_output(Self::AriaPressed)
                    .boxed(),
                EmbedAttributesName::AriaReadonly => crate::BValues::new_parser()
                    .map_output(Self::AriaReadonly)
                    .boxed(),
                EmbedAttributesName::AriaRelevant => crate::RelevantValues::new_parser()
                    .map_output(Self::AriaRelevant)
                    .boxed(),
                EmbedAttributesName::AriaRequired => crate::BValues::new_parser()
                    .map_output(Self::AriaRequired)
                    .boxed(),
                EmbedAttributesName::AriaRoledescription => String::new_parser()
                    .map_output(Self::AriaRoledescription)
                    .boxed(),
                EmbedAttributesName::AriaRowcount => {
                    String::new_parser().map_output(Self::AriaRowcount).boxed()
                }
                EmbedAttributesName::AriaRowindex => {
                    String::new_parser().map_output(Self::AriaRowindex).boxed()
                }
                EmbedAttributesName::AriaRowspan => {
                    String::new_parser().map_output(Self::AriaRowspan).boxed()
                }
                EmbedAttributesName::AriaSelected => crate::UValues::new_parser()
                    .map_output(Self::AriaSelected)
                    .boxed(),
                EmbedAttributesName::AriaSetsize => {
                    String::new_parser().map_output(Self::AriaSetsize).boxed()
                }
                EmbedAttributesName::AriaSort => crate::SortValues::new_parser()
                    .map_output(Self::AriaSort)
                    .boxed(),
                EmbedAttributesName::AriaValuemax => {
                    String::new_parser().map_output(Self::AriaValuemax).boxed()
                }
                EmbedAttributesName::AriaValuemin => {
                    String::new_parser().map_output(Self::AriaValuemin).boxed()
                }
                EmbedAttributesName::AriaValuenow => {
                    String::new_parser().map_output(Self::AriaValuenow).boxed()
                }
                EmbedAttributesName::AriaValuetext => {
                    String::new_parser().map_output(Self::AriaValuetext).boxed()
                }
                EmbedAttributesName::Autocapitalize => String::new_parser()
                    .map_output(Self::Autocapitalize)
                    .boxed(),
                EmbedAttributesName::Class => String::new_parser().map_output(Self::Class).boxed(),
                EmbedAttributesName::Contenteditable => String::new_parser()
                    .map_output(Self::Contenteditable)
                    .boxed(),
                EmbedAttributesName::Contextmenu => {
                    String::new_parser().map_output(Self::Contextmenu).boxed()
                }
                EmbedAttributesName::Dir => {
                    crate::DValues::new_parser().map_output(Self::Dir).boxed()
                }
                EmbedAttributesName::Draggable => crate::BValues::new_parser()
                    .map_output(Self::Draggable)
                    .boxed(),
                EmbedAttributesName::Dropzone => {
                    String::new_parser().map_output(Self::Dropzone).boxed()
                }
                EmbedAttributesName::Exportparts => {
                    String::new_parser().map_output(Self::Exportparts).boxed()
                }
                EmbedAttributesName::Height => {
                    String::new_parser().map_output(Self::Height).boxed()
                }
                EmbedAttributesName::Hidden => {
                    String::new_parser().map_output(Self::Hidden).boxed()
                }
                EmbedAttributesName::Id => String::new_parser().map_output(Self::Id).boxed(),
                EmbedAttributesName::Inputmode => {
                    String::new_parser().map_output(Self::Inputmode).boxed()
                }
                EmbedAttributesName::Is => String::new_parser().map_output(Self::Is).boxed(),
                EmbedAttributesName::Itemid => {
                    String::new_parser().map_output(Self::Itemid).boxed()
                }
                EmbedAttributesName::Itemprop => {
                    String::new_parser().map_output(Self::Itemprop).boxed()
                }
                EmbedAttributesName::Itemref => {
                    String::new_parser().map_output(Self::Itemref).boxed()
                }
                EmbedAttributesName::Itemscope => {
                    String::new_parser().map_output(Self::Itemscope).boxed()
                }
                EmbedAttributesName::Itemtype => {
                    String::new_parser().map_output(Self::Itemtype).boxed()
                }
                EmbedAttributesName::Lang => String::new_parser().map_output(Self::Lang).boxed(),
                EmbedAttributesName::Onabort => {
                    String::new_parser().map_output(Self::Onabort).boxed()
                }
                EmbedAttributesName::Onblur => {
                    String::new_parser().map_output(Self::Onblur).boxed()
                }
                EmbedAttributesName::Oncanplay => {
                    String::new_parser().map_output(Self::Oncanplay).boxed()
                }
                EmbedAttributesName::Oncanplaythrough => String::new_parser()
                    .map_output(Self::Oncanplaythrough)
                    .boxed(),
                EmbedAttributesName::Onchange => {
                    String::new_parser().map_output(Self::Onchange).boxed()
                }
                EmbedAttributesName::Onclick => {
                    String::new_parser().map_output(Self::Onclick).boxed()
                }
                EmbedAttributesName::Oncontextmenu => {
                    String::new_parser().map_output(Self::Oncontextmenu).boxed()
                }
                EmbedAttributesName::Ondblclick => {
                    String::new_parser().map_output(Self::Ondblclick).boxed()
                }
                EmbedAttributesName::Ondrag => {
                    String::new_parser().map_output(Self::Ondrag).boxed()
                }
                EmbedAttributesName::Ondragend => {
                    String::new_parser().map_output(Self::Ondragend).boxed()
                }
                EmbedAttributesName::Ondragenter => {
                    String::new_parser().map_output(Self::Ondragenter).boxed()
                }
                EmbedAttributesName::Ondragleave => {
                    String::new_parser().map_output(Self::Ondragleave).boxed()
                }
                EmbedAttributesName::Ondragover => {
                    String::new_parser().map_output(Self::Ondragover).boxed()
                }
                EmbedAttributesName::Ondragstart => {
                    String::new_parser().map_output(Self::Ondragstart).boxed()
                }
                EmbedAttributesName::Ondrop => {
                    String::new_parser().map_output(Self::Ondrop).boxed()
                }
                EmbedAttributesName::Ondurationchange => String::new_parser()
                    .map_output(Self::Ondurationchange)
                    .boxed(),
                EmbedAttributesName::Onemptied => {
                    String::new_parser().map_output(Self::Onemptied).boxed()
                }
                EmbedAttributesName::Onended => {
                    String::new_parser().map_output(Self::Onended).boxed()
                }
                EmbedAttributesName::Onerror => {
                    String::new_parser().map_output(Self::Onerror).boxed()
                }
                EmbedAttributesName::Onfocus => {
                    String::new_parser().map_output(Self::Onfocus).boxed()
                }
                EmbedAttributesName::Onformchange => {
                    String::new_parser().map_output(Self::Onformchange).boxed()
                }
                EmbedAttributesName::Onforminput => {
                    String::new_parser().map_output(Self::Onforminput).boxed()
                }
                EmbedAttributesName::Oninput => {
                    String::new_parser().map_output(Self::Oninput).boxed()
                }
                EmbedAttributesName::Oninvalid => {
                    String::new_parser().map_output(Self::Oninvalid).boxed()
                }
                EmbedAttributesName::Onkeydown => {
                    String::new_parser().map_output(Self::Onkeydown).boxed()
                }
                EmbedAttributesName::Onkeypress => {
                    String::new_parser().map_output(Self::Onkeypress).boxed()
                }
                EmbedAttributesName::Onkeyup => {
                    String::new_parser().map_output(Self::Onkeyup).boxed()
                }
                EmbedAttributesName::Onload => {
                    String::new_parser().map_output(Self::Onload).boxed()
                }
                EmbedAttributesName::Onloadeddata => {
                    String::new_parser().map_output(Self::Onloadeddata).boxed()
                }
                EmbedAttributesName::Onloadedmetadata => String::new_parser()
                    .map_output(Self::Onloadedmetadata)
                    .boxed(),
                EmbedAttributesName::Onloadstart => {
                    String::new_parser().map_output(Self::Onloadstart).boxed()
                }
                EmbedAttributesName::Onmousedown => {
                    String::new_parser().map_output(Self::Onmousedown).boxed()
                }
                EmbedAttributesName::Onmouseenter => {
                    String::new_parser().map_output(Self::Onmouseenter).boxed()
                }
                EmbedAttributesName::Onmouseleave => {
                    String::new_parser().map_output(Self::Onmouseleave).boxed()
                }
                EmbedAttributesName::Onmousemove => {
                    String::new_parser().map_output(Self::Onmousemove).boxed()
                }
                EmbedAttributesName::Onmouseout => {
                    String::new_parser().map_output(Self::Onmouseout).boxed()
                }
                EmbedAttributesName::Onmouseover => {
                    String::new_parser().map_output(Self::Onmouseover).boxed()
                }
                EmbedAttributesName::Onmouseup => {
                    String::new_parser().map_output(Self::Onmouseup).boxed()
                }
                EmbedAttributesName::Onmousewheel => {
                    String::new_parser().map_output(Self::Onmousewheel).boxed()
                }
                EmbedAttributesName::Onpause => {
                    String::new_parser().map_output(Self::Onpause).boxed()
                }
                EmbedAttributesName::Onplay => {
                    String::new_parser().map_output(Self::Onplay).boxed()
                }
                EmbedAttributesName::Onplaying => {
                    String::new_parser().map_output(Self::Onplaying).boxed()
                }
                EmbedAttributesName::Onpointercancel => String::new_parser()
                    .map_output(Self::Onpointercancel)
                    .boxed(),
                EmbedAttributesName::Onpointerdown => {
                    String::new_parser().map_output(Self::Onpointerdown).boxed()
                }
                EmbedAttributesName::Onpointerenter => String::new_parser()
                    .map_output(Self::Onpointerenter)
                    .boxed(),
                EmbedAttributesName::Onpointerleave => String::new_parser()
                    .map_output(Self::Onpointerleave)
                    .boxed(),
                EmbedAttributesName::Onpointerlockchange => String::new_parser()
                    .map_output(Self::Onpointerlockchange)
                    .boxed(),
                EmbedAttributesName::Onpointerlockerror => String::new_parser()
                    .map_output(Self::Onpointerlockerror)
                    .boxed(),
                EmbedAttributesName::Onpointermove => {
                    String::new_parser().map_output(Self::Onpointermove).boxed()
                }
                EmbedAttributesName::Onpointerout => {
                    String::new_parser().map_output(Self::Onpointerout).boxed()
                }
                EmbedAttributesName::Onpointerover => {
                    String::new_parser().map_output(Self::Onpointerover).boxed()
                }
                EmbedAttributesName::Onpointerup => {
                    String::new_parser().map_output(Self::Onpointerup).boxed()
                }
                EmbedAttributesName::Onprogress => {
                    String::new_parser().map_output(Self::Onprogress).boxed()
                }
                EmbedAttributesName::Onratechange => {
                    String::new_parser().map_output(Self::Onratechange).boxed()
                }
                EmbedAttributesName::Onreadystatechange => String::new_parser()
                    .map_output(Self::Onreadystatechange)
                    .boxed(),
                EmbedAttributesName::Onreset => {
                    String::new_parser().map_output(Self::Onreset).boxed()
                }
                EmbedAttributesName::Onresize => {
                    String::new_parser().map_output(Self::Onresize).boxed()
                }
                EmbedAttributesName::Onscroll => {
                    String::new_parser().map_output(Self::Onscroll).boxed()
                }
                EmbedAttributesName::Onseeked => {
                    String::new_parser().map_output(Self::Onseeked).boxed()
                }
                EmbedAttributesName::Onseeking => {
                    String::new_parser().map_output(Self::Onseeking).boxed()
                }
                EmbedAttributesName::Onselect => {
                    String::new_parser().map_output(Self::Onselect).boxed()
                }
                EmbedAttributesName::Onshow => {
                    String::new_parser().map_output(Self::Onshow).boxed()
                }
                EmbedAttributesName::Onstalled => {
                    String::new_parser().map_output(Self::Onstalled).boxed()
                }
                EmbedAttributesName::Onsubmit => {
                    String::new_parser().map_output(Self::Onsubmit).boxed()
                }
                EmbedAttributesName::Onsuspend => {
                    String::new_parser().map_output(Self::Onsuspend).boxed()
                }
                EmbedAttributesName::Ontimeupdate => {
                    String::new_parser().map_output(Self::Ontimeupdate).boxed()
                }
                EmbedAttributesName::Onvolumechange => String::new_parser()
                    .map_output(Self::Onvolumechange)
                    .boxed(),
                EmbedAttributesName::Onwaiting => {
                    String::new_parser().map_output(Self::Onwaiting).boxed()
                }
                EmbedAttributesName::Part => String::new_parser().map_output(Self::Part).boxed(),
                EmbedAttributesName::Role => crate::RolesValues::new_parser()
                    .map_output(Self::Role)
                    .boxed(),
                EmbedAttributesName::Slot => String::new_parser().map_output(Self::Slot).boxed(),
                EmbedAttributesName::Spellcheck => crate::BValues::new_parser()
                    .map_output(Self::Spellcheck)
                    .boxed(),
                EmbedAttributesName::Src => String::new_parser().map_output(Self::Src).boxed(),
                EmbedAttributesName::Style => String::new_parser().map_output(Self::Style).boxed(),
                EmbedAttributesName::Tabindex => {
                    String::new_parser().map_output(Self::Tabindex).boxed()
                }
                EmbedAttributesName::Title => String::new_parser().map_output(Self::Title).boxed(),
                EmbedAttributesName::Translate => crate::YValues::new_parser()
                    .map_output(Self::Translate)
                    .boxed(),
                EmbedAttributesName::Type => String::new_parser().map_output(Self::Type).boxed(),
                EmbedAttributesName::Width => String::new_parser().map_output(Self::Width).boxed(),
            })
            .map_output(|(_, attribute)| attribute)
    }
}
#[derive(Debug, Clone)]
pub struct Embed {
    attributes: Vec<EmbedAttributes>,
}
impl kalosm_sample::Parse for Embed {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        EmbedAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Embed { attributes })
    }
}
