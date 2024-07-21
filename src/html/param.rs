use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ParamAttributesName {
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
    #[parse(rename = " name=")]
    Name,
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
    #[parse(rename = " value=")]
    Value,
    #[parse(rename = " valuetype=")]
    Valuetype,
}
#[derive(Debug, Clone)]
pub enum ParamAttributes {
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
    Style(String),
    Tabindex(String),
    Title(String),
    Translate(crate::YValues),
    Type(String),
    Value(String),
    Valuetype(String),
}
impl kalosm_sample::Parse for ParamAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ParamAttributesName::new_parser()
            .then_lazy(|name| match name {
                ParamAttributesName::Accesskey => {
                    String::new_parser().map_output(Self::Accesskey).boxed()
                }
                ParamAttributesName::AriaActivedescendant => String::new_parser()
                    .map_output(Self::AriaActivedescendant)
                    .boxed(),
                ParamAttributesName::AriaAtomic => crate::BValues::new_parser()
                    .map_output(Self::AriaAtomic)
                    .boxed(),
                ParamAttributesName::AriaAutocomplete => crate::AutocompleteValues::new_parser()
                    .map_output(Self::AriaAutocomplete)
                    .boxed(),
                ParamAttributesName::AriaBusy => crate::BValues::new_parser()
                    .map_output(Self::AriaBusy)
                    .boxed(),
                ParamAttributesName::AriaChecked => crate::TristateValues::new_parser()
                    .map_output(Self::AriaChecked)
                    .boxed(),
                ParamAttributesName::AriaColcount => {
                    String::new_parser().map_output(Self::AriaColcount).boxed()
                }
                ParamAttributesName::AriaColindex => {
                    String::new_parser().map_output(Self::AriaColindex).boxed()
                }
                ParamAttributesName::AriaColspan => {
                    String::new_parser().map_output(Self::AriaColspan).boxed()
                }
                ParamAttributesName::AriaControls => {
                    String::new_parser().map_output(Self::AriaControls).boxed()
                }
                ParamAttributesName::AriaCurrent => crate::CurrentValues::new_parser()
                    .map_output(Self::AriaCurrent)
                    .boxed(),
                ParamAttributesName::AriaDescribedby => String::new_parser()
                    .map_output(Self::AriaDescribedby)
                    .boxed(),
                ParamAttributesName::AriaDetails => {
                    String::new_parser().map_output(Self::AriaDetails).boxed()
                }
                ParamAttributesName::AriaDisabled => crate::BValues::new_parser()
                    .map_output(Self::AriaDisabled)
                    .boxed(),
                ParamAttributesName::AriaDropeffect => crate::DropeffectValues::new_parser()
                    .map_output(Self::AriaDropeffect)
                    .boxed(),
                ParamAttributesName::AriaErrormessage => String::new_parser()
                    .map_output(Self::AriaErrormessage)
                    .boxed(),
                ParamAttributesName::AriaExpanded => crate::UValues::new_parser()
                    .map_output(Self::AriaExpanded)
                    .boxed(),
                ParamAttributesName::AriaFlowto => {
                    String::new_parser().map_output(Self::AriaFlowto).boxed()
                }
                ParamAttributesName::AriaGrabbed => crate::UValues::new_parser()
                    .map_output(Self::AriaGrabbed)
                    .boxed(),
                ParamAttributesName::AriaHaspopup => crate::HaspopupValues::new_parser()
                    .map_output(Self::AriaHaspopup)
                    .boxed(),
                ParamAttributesName::AriaHidden => crate::BValues::new_parser()
                    .map_output(Self::AriaHidden)
                    .boxed(),
                ParamAttributesName::AriaInvalid => crate::InvalidValues::new_parser()
                    .map_output(Self::AriaInvalid)
                    .boxed(),
                ParamAttributesName::AriaKeyshortcuts => String::new_parser()
                    .map_output(Self::AriaKeyshortcuts)
                    .boxed(),
                ParamAttributesName::AriaLabel => {
                    String::new_parser().map_output(Self::AriaLabel).boxed()
                }
                ParamAttributesName::AriaLabelledby => String::new_parser()
                    .map_output(Self::AriaLabelledby)
                    .boxed(),
                ParamAttributesName::AriaLevel => {
                    String::new_parser().map_output(Self::AriaLevel).boxed()
                }
                ParamAttributesName::AriaLive => crate::LiveValues::new_parser()
                    .map_output(Self::AriaLive)
                    .boxed(),
                ParamAttributesName::AriaModal => crate::BValues::new_parser()
                    .map_output(Self::AriaModal)
                    .boxed(),
                ParamAttributesName::AriaMultiline => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiline)
                    .boxed(),
                ParamAttributesName::AriaMultiselectable => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiselectable)
                    .boxed(),
                ParamAttributesName::AriaOrientation => crate::OrientationValues::new_parser()
                    .map_output(Self::AriaOrientation)
                    .boxed(),
                ParamAttributesName::AriaOwns => {
                    String::new_parser().map_output(Self::AriaOwns).boxed()
                }
                ParamAttributesName::AriaPlaceholder => String::new_parser()
                    .map_output(Self::AriaPlaceholder)
                    .boxed(),
                ParamAttributesName::AriaPosinset => {
                    String::new_parser().map_output(Self::AriaPosinset).boxed()
                }
                ParamAttributesName::AriaPressed => crate::TristateValues::new_parser()
                    .map_output(Self::AriaPressed)
                    .boxed(),
                ParamAttributesName::AriaReadonly => crate::BValues::new_parser()
                    .map_output(Self::AriaReadonly)
                    .boxed(),
                ParamAttributesName::AriaRelevant => crate::RelevantValues::new_parser()
                    .map_output(Self::AriaRelevant)
                    .boxed(),
                ParamAttributesName::AriaRequired => crate::BValues::new_parser()
                    .map_output(Self::AriaRequired)
                    .boxed(),
                ParamAttributesName::AriaRoledescription => String::new_parser()
                    .map_output(Self::AriaRoledescription)
                    .boxed(),
                ParamAttributesName::AriaRowcount => {
                    String::new_parser().map_output(Self::AriaRowcount).boxed()
                }
                ParamAttributesName::AriaRowindex => {
                    String::new_parser().map_output(Self::AriaRowindex).boxed()
                }
                ParamAttributesName::AriaRowspan => {
                    String::new_parser().map_output(Self::AriaRowspan).boxed()
                }
                ParamAttributesName::AriaSelected => crate::UValues::new_parser()
                    .map_output(Self::AriaSelected)
                    .boxed(),
                ParamAttributesName::AriaSetsize => {
                    String::new_parser().map_output(Self::AriaSetsize).boxed()
                }
                ParamAttributesName::AriaSort => crate::SortValues::new_parser()
                    .map_output(Self::AriaSort)
                    .boxed(),
                ParamAttributesName::AriaValuemax => {
                    String::new_parser().map_output(Self::AriaValuemax).boxed()
                }
                ParamAttributesName::AriaValuemin => {
                    String::new_parser().map_output(Self::AriaValuemin).boxed()
                }
                ParamAttributesName::AriaValuenow => {
                    String::new_parser().map_output(Self::AriaValuenow).boxed()
                }
                ParamAttributesName::AriaValuetext => {
                    String::new_parser().map_output(Self::AriaValuetext).boxed()
                }
                ParamAttributesName::Autocapitalize => String::new_parser()
                    .map_output(Self::Autocapitalize)
                    .boxed(),
                ParamAttributesName::Class => String::new_parser().map_output(Self::Class).boxed(),
                ParamAttributesName::Contenteditable => String::new_parser()
                    .map_output(Self::Contenteditable)
                    .boxed(),
                ParamAttributesName::Contextmenu => {
                    String::new_parser().map_output(Self::Contextmenu).boxed()
                }
                ParamAttributesName::Dir => {
                    crate::DValues::new_parser().map_output(Self::Dir).boxed()
                }
                ParamAttributesName::Draggable => crate::BValues::new_parser()
                    .map_output(Self::Draggable)
                    .boxed(),
                ParamAttributesName::Dropzone => {
                    String::new_parser().map_output(Self::Dropzone).boxed()
                }
                ParamAttributesName::Exportparts => {
                    String::new_parser().map_output(Self::Exportparts).boxed()
                }
                ParamAttributesName::Hidden => {
                    String::new_parser().map_output(Self::Hidden).boxed()
                }
                ParamAttributesName::Id => String::new_parser().map_output(Self::Id).boxed(),
                ParamAttributesName::Inputmode => {
                    String::new_parser().map_output(Self::Inputmode).boxed()
                }
                ParamAttributesName::Is => String::new_parser().map_output(Self::Is).boxed(),
                ParamAttributesName::Itemid => {
                    String::new_parser().map_output(Self::Itemid).boxed()
                }
                ParamAttributesName::Itemprop => {
                    String::new_parser().map_output(Self::Itemprop).boxed()
                }
                ParamAttributesName::Itemref => {
                    String::new_parser().map_output(Self::Itemref).boxed()
                }
                ParamAttributesName::Itemscope => {
                    String::new_parser().map_output(Self::Itemscope).boxed()
                }
                ParamAttributesName::Itemtype => {
                    String::new_parser().map_output(Self::Itemtype).boxed()
                }
                ParamAttributesName::Lang => String::new_parser().map_output(Self::Lang).boxed(),
                ParamAttributesName::Name => String::new_parser().map_output(Self::Name).boxed(),
                ParamAttributesName::Onabort => {
                    String::new_parser().map_output(Self::Onabort).boxed()
                }
                ParamAttributesName::Onblur => {
                    String::new_parser().map_output(Self::Onblur).boxed()
                }
                ParamAttributesName::Oncanplay => {
                    String::new_parser().map_output(Self::Oncanplay).boxed()
                }
                ParamAttributesName::Oncanplaythrough => String::new_parser()
                    .map_output(Self::Oncanplaythrough)
                    .boxed(),
                ParamAttributesName::Onchange => {
                    String::new_parser().map_output(Self::Onchange).boxed()
                }
                ParamAttributesName::Onclick => {
                    String::new_parser().map_output(Self::Onclick).boxed()
                }
                ParamAttributesName::Oncontextmenu => {
                    String::new_parser().map_output(Self::Oncontextmenu).boxed()
                }
                ParamAttributesName::Ondblclick => {
                    String::new_parser().map_output(Self::Ondblclick).boxed()
                }
                ParamAttributesName::Ondrag => {
                    String::new_parser().map_output(Self::Ondrag).boxed()
                }
                ParamAttributesName::Ondragend => {
                    String::new_parser().map_output(Self::Ondragend).boxed()
                }
                ParamAttributesName::Ondragenter => {
                    String::new_parser().map_output(Self::Ondragenter).boxed()
                }
                ParamAttributesName::Ondragleave => {
                    String::new_parser().map_output(Self::Ondragleave).boxed()
                }
                ParamAttributesName::Ondragover => {
                    String::new_parser().map_output(Self::Ondragover).boxed()
                }
                ParamAttributesName::Ondragstart => {
                    String::new_parser().map_output(Self::Ondragstart).boxed()
                }
                ParamAttributesName::Ondrop => {
                    String::new_parser().map_output(Self::Ondrop).boxed()
                }
                ParamAttributesName::Ondurationchange => String::new_parser()
                    .map_output(Self::Ondurationchange)
                    .boxed(),
                ParamAttributesName::Onemptied => {
                    String::new_parser().map_output(Self::Onemptied).boxed()
                }
                ParamAttributesName::Onended => {
                    String::new_parser().map_output(Self::Onended).boxed()
                }
                ParamAttributesName::Onerror => {
                    String::new_parser().map_output(Self::Onerror).boxed()
                }
                ParamAttributesName::Onfocus => {
                    String::new_parser().map_output(Self::Onfocus).boxed()
                }
                ParamAttributesName::Onformchange => {
                    String::new_parser().map_output(Self::Onformchange).boxed()
                }
                ParamAttributesName::Onforminput => {
                    String::new_parser().map_output(Self::Onforminput).boxed()
                }
                ParamAttributesName::Oninput => {
                    String::new_parser().map_output(Self::Oninput).boxed()
                }
                ParamAttributesName::Oninvalid => {
                    String::new_parser().map_output(Self::Oninvalid).boxed()
                }
                ParamAttributesName::Onkeydown => {
                    String::new_parser().map_output(Self::Onkeydown).boxed()
                }
                ParamAttributesName::Onkeypress => {
                    String::new_parser().map_output(Self::Onkeypress).boxed()
                }
                ParamAttributesName::Onkeyup => {
                    String::new_parser().map_output(Self::Onkeyup).boxed()
                }
                ParamAttributesName::Onload => {
                    String::new_parser().map_output(Self::Onload).boxed()
                }
                ParamAttributesName::Onloadeddata => {
                    String::new_parser().map_output(Self::Onloadeddata).boxed()
                }
                ParamAttributesName::Onloadedmetadata => String::new_parser()
                    .map_output(Self::Onloadedmetadata)
                    .boxed(),
                ParamAttributesName::Onloadstart => {
                    String::new_parser().map_output(Self::Onloadstart).boxed()
                }
                ParamAttributesName::Onmousedown => {
                    String::new_parser().map_output(Self::Onmousedown).boxed()
                }
                ParamAttributesName::Onmouseenter => {
                    String::new_parser().map_output(Self::Onmouseenter).boxed()
                }
                ParamAttributesName::Onmouseleave => {
                    String::new_parser().map_output(Self::Onmouseleave).boxed()
                }
                ParamAttributesName::Onmousemove => {
                    String::new_parser().map_output(Self::Onmousemove).boxed()
                }
                ParamAttributesName::Onmouseout => {
                    String::new_parser().map_output(Self::Onmouseout).boxed()
                }
                ParamAttributesName::Onmouseover => {
                    String::new_parser().map_output(Self::Onmouseover).boxed()
                }
                ParamAttributesName::Onmouseup => {
                    String::new_parser().map_output(Self::Onmouseup).boxed()
                }
                ParamAttributesName::Onmousewheel => {
                    String::new_parser().map_output(Self::Onmousewheel).boxed()
                }
                ParamAttributesName::Onpause => {
                    String::new_parser().map_output(Self::Onpause).boxed()
                }
                ParamAttributesName::Onplay => {
                    String::new_parser().map_output(Self::Onplay).boxed()
                }
                ParamAttributesName::Onplaying => {
                    String::new_parser().map_output(Self::Onplaying).boxed()
                }
                ParamAttributesName::Onpointercancel => String::new_parser()
                    .map_output(Self::Onpointercancel)
                    .boxed(),
                ParamAttributesName::Onpointerdown => {
                    String::new_parser().map_output(Self::Onpointerdown).boxed()
                }
                ParamAttributesName::Onpointerenter => String::new_parser()
                    .map_output(Self::Onpointerenter)
                    .boxed(),
                ParamAttributesName::Onpointerleave => String::new_parser()
                    .map_output(Self::Onpointerleave)
                    .boxed(),
                ParamAttributesName::Onpointerlockchange => String::new_parser()
                    .map_output(Self::Onpointerlockchange)
                    .boxed(),
                ParamAttributesName::Onpointerlockerror => String::new_parser()
                    .map_output(Self::Onpointerlockerror)
                    .boxed(),
                ParamAttributesName::Onpointermove => {
                    String::new_parser().map_output(Self::Onpointermove).boxed()
                }
                ParamAttributesName::Onpointerout => {
                    String::new_parser().map_output(Self::Onpointerout).boxed()
                }
                ParamAttributesName::Onpointerover => {
                    String::new_parser().map_output(Self::Onpointerover).boxed()
                }
                ParamAttributesName::Onpointerup => {
                    String::new_parser().map_output(Self::Onpointerup).boxed()
                }
                ParamAttributesName::Onprogress => {
                    String::new_parser().map_output(Self::Onprogress).boxed()
                }
                ParamAttributesName::Onratechange => {
                    String::new_parser().map_output(Self::Onratechange).boxed()
                }
                ParamAttributesName::Onreadystatechange => String::new_parser()
                    .map_output(Self::Onreadystatechange)
                    .boxed(),
                ParamAttributesName::Onreset => {
                    String::new_parser().map_output(Self::Onreset).boxed()
                }
                ParamAttributesName::Onresize => {
                    String::new_parser().map_output(Self::Onresize).boxed()
                }
                ParamAttributesName::Onscroll => {
                    String::new_parser().map_output(Self::Onscroll).boxed()
                }
                ParamAttributesName::Onseeked => {
                    String::new_parser().map_output(Self::Onseeked).boxed()
                }
                ParamAttributesName::Onseeking => {
                    String::new_parser().map_output(Self::Onseeking).boxed()
                }
                ParamAttributesName::Onselect => {
                    String::new_parser().map_output(Self::Onselect).boxed()
                }
                ParamAttributesName::Onshow => {
                    String::new_parser().map_output(Self::Onshow).boxed()
                }
                ParamAttributesName::Onstalled => {
                    String::new_parser().map_output(Self::Onstalled).boxed()
                }
                ParamAttributesName::Onsubmit => {
                    String::new_parser().map_output(Self::Onsubmit).boxed()
                }
                ParamAttributesName::Onsuspend => {
                    String::new_parser().map_output(Self::Onsuspend).boxed()
                }
                ParamAttributesName::Ontimeupdate => {
                    String::new_parser().map_output(Self::Ontimeupdate).boxed()
                }
                ParamAttributesName::Onvolumechange => String::new_parser()
                    .map_output(Self::Onvolumechange)
                    .boxed(),
                ParamAttributesName::Onwaiting => {
                    String::new_parser().map_output(Self::Onwaiting).boxed()
                }
                ParamAttributesName::Part => String::new_parser().map_output(Self::Part).boxed(),
                ParamAttributesName::Role => crate::RolesValues::new_parser()
                    .map_output(Self::Role)
                    .boxed(),
                ParamAttributesName::Slot => String::new_parser().map_output(Self::Slot).boxed(),
                ParamAttributesName::Spellcheck => crate::BValues::new_parser()
                    .map_output(Self::Spellcheck)
                    .boxed(),
                ParamAttributesName::Style => String::new_parser().map_output(Self::Style).boxed(),
                ParamAttributesName::Tabindex => {
                    String::new_parser().map_output(Self::Tabindex).boxed()
                }
                ParamAttributesName::Title => String::new_parser().map_output(Self::Title).boxed(),
                ParamAttributesName::Translate => crate::YValues::new_parser()
                    .map_output(Self::Translate)
                    .boxed(),
                ParamAttributesName::Type => String::new_parser().map_output(Self::Type).boxed(),
                ParamAttributesName::Value => String::new_parser().map_output(Self::Value).boxed(),
                ParamAttributesName::Valuetype => {
                    String::new_parser().map_output(Self::Valuetype).boxed()
                }
            })
            .map_output(|(_, attribute)| attribute)
    }
}
#[derive(Debug, Clone)]
pub struct Param {
    attributes: Vec<ParamAttributes>,
}
impl kalosm_sample::Parse for Param {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ParamAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Param { attributes })
    }
}
