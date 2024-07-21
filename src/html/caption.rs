use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum CaptionAttributesName {
    #[parse(rename = " accesskey=")]
    Accesskey,
    #[parse(rename = " align=")]
    Align,
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
}
#[derive(Debug, Clone)]
pub enum CaptionAttributes {
    Accesskey(String),
    Align(String),
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
}
impl kalosm_sample::Parse for CaptionAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        CaptionAttributesName::new_parser()
            .then_lazy(|name| match name {
                CaptionAttributesName::Accesskey => {
                    String::new_parser().map_output(Self::Accesskey).boxed()
                }
                CaptionAttributesName::Align => {
                    String::new_parser().map_output(Self::Align).boxed()
                }
                CaptionAttributesName::AriaActivedescendant => String::new_parser()
                    .map_output(Self::AriaActivedescendant)
                    .boxed(),
                CaptionAttributesName::AriaAtomic => crate::BValues::new_parser()
                    .map_output(Self::AriaAtomic)
                    .boxed(),
                CaptionAttributesName::AriaAutocomplete => crate::AutocompleteValues::new_parser()
                    .map_output(Self::AriaAutocomplete)
                    .boxed(),
                CaptionAttributesName::AriaBusy => crate::BValues::new_parser()
                    .map_output(Self::AriaBusy)
                    .boxed(),
                CaptionAttributesName::AriaChecked => crate::TristateValues::new_parser()
                    .map_output(Self::AriaChecked)
                    .boxed(),
                CaptionAttributesName::AriaColcount => {
                    String::new_parser().map_output(Self::AriaColcount).boxed()
                }
                CaptionAttributesName::AriaColindex => {
                    String::new_parser().map_output(Self::AriaColindex).boxed()
                }
                CaptionAttributesName::AriaColspan => {
                    String::new_parser().map_output(Self::AriaColspan).boxed()
                }
                CaptionAttributesName::AriaControls => {
                    String::new_parser().map_output(Self::AriaControls).boxed()
                }
                CaptionAttributesName::AriaCurrent => crate::CurrentValues::new_parser()
                    .map_output(Self::AriaCurrent)
                    .boxed(),
                CaptionAttributesName::AriaDescribedby => String::new_parser()
                    .map_output(Self::AriaDescribedby)
                    .boxed(),
                CaptionAttributesName::AriaDetails => {
                    String::new_parser().map_output(Self::AriaDetails).boxed()
                }
                CaptionAttributesName::AriaDisabled => crate::BValues::new_parser()
                    .map_output(Self::AriaDisabled)
                    .boxed(),
                CaptionAttributesName::AriaDropeffect => crate::DropeffectValues::new_parser()
                    .map_output(Self::AriaDropeffect)
                    .boxed(),
                CaptionAttributesName::AriaErrormessage => String::new_parser()
                    .map_output(Self::AriaErrormessage)
                    .boxed(),
                CaptionAttributesName::AriaExpanded => crate::UValues::new_parser()
                    .map_output(Self::AriaExpanded)
                    .boxed(),
                CaptionAttributesName::AriaFlowto => {
                    String::new_parser().map_output(Self::AriaFlowto).boxed()
                }
                CaptionAttributesName::AriaGrabbed => crate::UValues::new_parser()
                    .map_output(Self::AriaGrabbed)
                    .boxed(),
                CaptionAttributesName::AriaHaspopup => crate::HaspopupValues::new_parser()
                    .map_output(Self::AriaHaspopup)
                    .boxed(),
                CaptionAttributesName::AriaHidden => crate::BValues::new_parser()
                    .map_output(Self::AriaHidden)
                    .boxed(),
                CaptionAttributesName::AriaInvalid => crate::InvalidValues::new_parser()
                    .map_output(Self::AriaInvalid)
                    .boxed(),
                CaptionAttributesName::AriaKeyshortcuts => String::new_parser()
                    .map_output(Self::AriaKeyshortcuts)
                    .boxed(),
                CaptionAttributesName::AriaLabel => {
                    String::new_parser().map_output(Self::AriaLabel).boxed()
                }
                CaptionAttributesName::AriaLabelledby => String::new_parser()
                    .map_output(Self::AriaLabelledby)
                    .boxed(),
                CaptionAttributesName::AriaLevel => {
                    String::new_parser().map_output(Self::AriaLevel).boxed()
                }
                CaptionAttributesName::AriaLive => crate::LiveValues::new_parser()
                    .map_output(Self::AriaLive)
                    .boxed(),
                CaptionAttributesName::AriaModal => crate::BValues::new_parser()
                    .map_output(Self::AriaModal)
                    .boxed(),
                CaptionAttributesName::AriaMultiline => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiline)
                    .boxed(),
                CaptionAttributesName::AriaMultiselectable => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiselectable)
                    .boxed(),
                CaptionAttributesName::AriaOrientation => crate::OrientationValues::new_parser()
                    .map_output(Self::AriaOrientation)
                    .boxed(),
                CaptionAttributesName::AriaOwns => {
                    String::new_parser().map_output(Self::AriaOwns).boxed()
                }
                CaptionAttributesName::AriaPlaceholder => String::new_parser()
                    .map_output(Self::AriaPlaceholder)
                    .boxed(),
                CaptionAttributesName::AriaPosinset => {
                    String::new_parser().map_output(Self::AriaPosinset).boxed()
                }
                CaptionAttributesName::AriaPressed => crate::TristateValues::new_parser()
                    .map_output(Self::AriaPressed)
                    .boxed(),
                CaptionAttributesName::AriaReadonly => crate::BValues::new_parser()
                    .map_output(Self::AriaReadonly)
                    .boxed(),
                CaptionAttributesName::AriaRelevant => crate::RelevantValues::new_parser()
                    .map_output(Self::AriaRelevant)
                    .boxed(),
                CaptionAttributesName::AriaRequired => crate::BValues::new_parser()
                    .map_output(Self::AriaRequired)
                    .boxed(),
                CaptionAttributesName::AriaRoledescription => String::new_parser()
                    .map_output(Self::AriaRoledescription)
                    .boxed(),
                CaptionAttributesName::AriaRowcount => {
                    String::new_parser().map_output(Self::AriaRowcount).boxed()
                }
                CaptionAttributesName::AriaRowindex => {
                    String::new_parser().map_output(Self::AriaRowindex).boxed()
                }
                CaptionAttributesName::AriaRowspan => {
                    String::new_parser().map_output(Self::AriaRowspan).boxed()
                }
                CaptionAttributesName::AriaSelected => crate::UValues::new_parser()
                    .map_output(Self::AriaSelected)
                    .boxed(),
                CaptionAttributesName::AriaSetsize => {
                    String::new_parser().map_output(Self::AriaSetsize).boxed()
                }
                CaptionAttributesName::AriaSort => crate::SortValues::new_parser()
                    .map_output(Self::AriaSort)
                    .boxed(),
                CaptionAttributesName::AriaValuemax => {
                    String::new_parser().map_output(Self::AriaValuemax).boxed()
                }
                CaptionAttributesName::AriaValuemin => {
                    String::new_parser().map_output(Self::AriaValuemin).boxed()
                }
                CaptionAttributesName::AriaValuenow => {
                    String::new_parser().map_output(Self::AriaValuenow).boxed()
                }
                CaptionAttributesName::AriaValuetext => {
                    String::new_parser().map_output(Self::AriaValuetext).boxed()
                }
                CaptionAttributesName::Autocapitalize => String::new_parser()
                    .map_output(Self::Autocapitalize)
                    .boxed(),
                CaptionAttributesName::Class => {
                    String::new_parser().map_output(Self::Class).boxed()
                }
                CaptionAttributesName::Contenteditable => String::new_parser()
                    .map_output(Self::Contenteditable)
                    .boxed(),
                CaptionAttributesName::Contextmenu => {
                    String::new_parser().map_output(Self::Contextmenu).boxed()
                }
                CaptionAttributesName::Dir => {
                    crate::DValues::new_parser().map_output(Self::Dir).boxed()
                }
                CaptionAttributesName::Draggable => crate::BValues::new_parser()
                    .map_output(Self::Draggable)
                    .boxed(),
                CaptionAttributesName::Dropzone => {
                    String::new_parser().map_output(Self::Dropzone).boxed()
                }
                CaptionAttributesName::Exportparts => {
                    String::new_parser().map_output(Self::Exportparts).boxed()
                }
                CaptionAttributesName::Hidden => {
                    String::new_parser().map_output(Self::Hidden).boxed()
                }
                CaptionAttributesName::Id => String::new_parser().map_output(Self::Id).boxed(),
                CaptionAttributesName::Inputmode => {
                    String::new_parser().map_output(Self::Inputmode).boxed()
                }
                CaptionAttributesName::Is => String::new_parser().map_output(Self::Is).boxed(),
                CaptionAttributesName::Itemid => {
                    String::new_parser().map_output(Self::Itemid).boxed()
                }
                CaptionAttributesName::Itemprop => {
                    String::new_parser().map_output(Self::Itemprop).boxed()
                }
                CaptionAttributesName::Itemref => {
                    String::new_parser().map_output(Self::Itemref).boxed()
                }
                CaptionAttributesName::Itemscope => {
                    String::new_parser().map_output(Self::Itemscope).boxed()
                }
                CaptionAttributesName::Itemtype => {
                    String::new_parser().map_output(Self::Itemtype).boxed()
                }
                CaptionAttributesName::Lang => String::new_parser().map_output(Self::Lang).boxed(),
                CaptionAttributesName::Onabort => {
                    String::new_parser().map_output(Self::Onabort).boxed()
                }
                CaptionAttributesName::Onblur => {
                    String::new_parser().map_output(Self::Onblur).boxed()
                }
                CaptionAttributesName::Oncanplay => {
                    String::new_parser().map_output(Self::Oncanplay).boxed()
                }
                CaptionAttributesName::Oncanplaythrough => String::new_parser()
                    .map_output(Self::Oncanplaythrough)
                    .boxed(),
                CaptionAttributesName::Onchange => {
                    String::new_parser().map_output(Self::Onchange).boxed()
                }
                CaptionAttributesName::Onclick => {
                    String::new_parser().map_output(Self::Onclick).boxed()
                }
                CaptionAttributesName::Oncontextmenu => {
                    String::new_parser().map_output(Self::Oncontextmenu).boxed()
                }
                CaptionAttributesName::Ondblclick => {
                    String::new_parser().map_output(Self::Ondblclick).boxed()
                }
                CaptionAttributesName::Ondrag => {
                    String::new_parser().map_output(Self::Ondrag).boxed()
                }
                CaptionAttributesName::Ondragend => {
                    String::new_parser().map_output(Self::Ondragend).boxed()
                }
                CaptionAttributesName::Ondragenter => {
                    String::new_parser().map_output(Self::Ondragenter).boxed()
                }
                CaptionAttributesName::Ondragleave => {
                    String::new_parser().map_output(Self::Ondragleave).boxed()
                }
                CaptionAttributesName::Ondragover => {
                    String::new_parser().map_output(Self::Ondragover).boxed()
                }
                CaptionAttributesName::Ondragstart => {
                    String::new_parser().map_output(Self::Ondragstart).boxed()
                }
                CaptionAttributesName::Ondrop => {
                    String::new_parser().map_output(Self::Ondrop).boxed()
                }
                CaptionAttributesName::Ondurationchange => String::new_parser()
                    .map_output(Self::Ondurationchange)
                    .boxed(),
                CaptionAttributesName::Onemptied => {
                    String::new_parser().map_output(Self::Onemptied).boxed()
                }
                CaptionAttributesName::Onended => {
                    String::new_parser().map_output(Self::Onended).boxed()
                }
                CaptionAttributesName::Onerror => {
                    String::new_parser().map_output(Self::Onerror).boxed()
                }
                CaptionAttributesName::Onfocus => {
                    String::new_parser().map_output(Self::Onfocus).boxed()
                }
                CaptionAttributesName::Onformchange => {
                    String::new_parser().map_output(Self::Onformchange).boxed()
                }
                CaptionAttributesName::Onforminput => {
                    String::new_parser().map_output(Self::Onforminput).boxed()
                }
                CaptionAttributesName::Oninput => {
                    String::new_parser().map_output(Self::Oninput).boxed()
                }
                CaptionAttributesName::Oninvalid => {
                    String::new_parser().map_output(Self::Oninvalid).boxed()
                }
                CaptionAttributesName::Onkeydown => {
                    String::new_parser().map_output(Self::Onkeydown).boxed()
                }
                CaptionAttributesName::Onkeypress => {
                    String::new_parser().map_output(Self::Onkeypress).boxed()
                }
                CaptionAttributesName::Onkeyup => {
                    String::new_parser().map_output(Self::Onkeyup).boxed()
                }
                CaptionAttributesName::Onload => {
                    String::new_parser().map_output(Self::Onload).boxed()
                }
                CaptionAttributesName::Onloadeddata => {
                    String::new_parser().map_output(Self::Onloadeddata).boxed()
                }
                CaptionAttributesName::Onloadedmetadata => String::new_parser()
                    .map_output(Self::Onloadedmetadata)
                    .boxed(),
                CaptionAttributesName::Onloadstart => {
                    String::new_parser().map_output(Self::Onloadstart).boxed()
                }
                CaptionAttributesName::Onmousedown => {
                    String::new_parser().map_output(Self::Onmousedown).boxed()
                }
                CaptionAttributesName::Onmouseenter => {
                    String::new_parser().map_output(Self::Onmouseenter).boxed()
                }
                CaptionAttributesName::Onmouseleave => {
                    String::new_parser().map_output(Self::Onmouseleave).boxed()
                }
                CaptionAttributesName::Onmousemove => {
                    String::new_parser().map_output(Self::Onmousemove).boxed()
                }
                CaptionAttributesName::Onmouseout => {
                    String::new_parser().map_output(Self::Onmouseout).boxed()
                }
                CaptionAttributesName::Onmouseover => {
                    String::new_parser().map_output(Self::Onmouseover).boxed()
                }
                CaptionAttributesName::Onmouseup => {
                    String::new_parser().map_output(Self::Onmouseup).boxed()
                }
                CaptionAttributesName::Onmousewheel => {
                    String::new_parser().map_output(Self::Onmousewheel).boxed()
                }
                CaptionAttributesName::Onpause => {
                    String::new_parser().map_output(Self::Onpause).boxed()
                }
                CaptionAttributesName::Onplay => {
                    String::new_parser().map_output(Self::Onplay).boxed()
                }
                CaptionAttributesName::Onplaying => {
                    String::new_parser().map_output(Self::Onplaying).boxed()
                }
                CaptionAttributesName::Onpointercancel => String::new_parser()
                    .map_output(Self::Onpointercancel)
                    .boxed(),
                CaptionAttributesName::Onpointerdown => {
                    String::new_parser().map_output(Self::Onpointerdown).boxed()
                }
                CaptionAttributesName::Onpointerenter => String::new_parser()
                    .map_output(Self::Onpointerenter)
                    .boxed(),
                CaptionAttributesName::Onpointerleave => String::new_parser()
                    .map_output(Self::Onpointerleave)
                    .boxed(),
                CaptionAttributesName::Onpointerlockchange => String::new_parser()
                    .map_output(Self::Onpointerlockchange)
                    .boxed(),
                CaptionAttributesName::Onpointerlockerror => String::new_parser()
                    .map_output(Self::Onpointerlockerror)
                    .boxed(),
                CaptionAttributesName::Onpointermove => {
                    String::new_parser().map_output(Self::Onpointermove).boxed()
                }
                CaptionAttributesName::Onpointerout => {
                    String::new_parser().map_output(Self::Onpointerout).boxed()
                }
                CaptionAttributesName::Onpointerover => {
                    String::new_parser().map_output(Self::Onpointerover).boxed()
                }
                CaptionAttributesName::Onpointerup => {
                    String::new_parser().map_output(Self::Onpointerup).boxed()
                }
                CaptionAttributesName::Onprogress => {
                    String::new_parser().map_output(Self::Onprogress).boxed()
                }
                CaptionAttributesName::Onratechange => {
                    String::new_parser().map_output(Self::Onratechange).boxed()
                }
                CaptionAttributesName::Onreadystatechange => String::new_parser()
                    .map_output(Self::Onreadystatechange)
                    .boxed(),
                CaptionAttributesName::Onreset => {
                    String::new_parser().map_output(Self::Onreset).boxed()
                }
                CaptionAttributesName::Onresize => {
                    String::new_parser().map_output(Self::Onresize).boxed()
                }
                CaptionAttributesName::Onscroll => {
                    String::new_parser().map_output(Self::Onscroll).boxed()
                }
                CaptionAttributesName::Onseeked => {
                    String::new_parser().map_output(Self::Onseeked).boxed()
                }
                CaptionAttributesName::Onseeking => {
                    String::new_parser().map_output(Self::Onseeking).boxed()
                }
                CaptionAttributesName::Onselect => {
                    String::new_parser().map_output(Self::Onselect).boxed()
                }
                CaptionAttributesName::Onshow => {
                    String::new_parser().map_output(Self::Onshow).boxed()
                }
                CaptionAttributesName::Onstalled => {
                    String::new_parser().map_output(Self::Onstalled).boxed()
                }
                CaptionAttributesName::Onsubmit => {
                    String::new_parser().map_output(Self::Onsubmit).boxed()
                }
                CaptionAttributesName::Onsuspend => {
                    String::new_parser().map_output(Self::Onsuspend).boxed()
                }
                CaptionAttributesName::Ontimeupdate => {
                    String::new_parser().map_output(Self::Ontimeupdate).boxed()
                }
                CaptionAttributesName::Onvolumechange => String::new_parser()
                    .map_output(Self::Onvolumechange)
                    .boxed(),
                CaptionAttributesName::Onwaiting => {
                    String::new_parser().map_output(Self::Onwaiting).boxed()
                }
                CaptionAttributesName::Part => String::new_parser().map_output(Self::Part).boxed(),
                CaptionAttributesName::Role => crate::RolesValues::new_parser()
                    .map_output(Self::Role)
                    .boxed(),
                CaptionAttributesName::Slot => String::new_parser().map_output(Self::Slot).boxed(),
                CaptionAttributesName::Spellcheck => crate::BValues::new_parser()
                    .map_output(Self::Spellcheck)
                    .boxed(),
                CaptionAttributesName::Style => {
                    String::new_parser().map_output(Self::Style).boxed()
                }
                CaptionAttributesName::Tabindex => {
                    String::new_parser().map_output(Self::Tabindex).boxed()
                }
                CaptionAttributesName::Title => {
                    String::new_parser().map_output(Self::Title).boxed()
                }
                CaptionAttributesName::Translate => crate::YValues::new_parser()
                    .map_output(Self::Translate)
                    .boxed(),
            })
            .map_output(|(_, attribute)| attribute)
    }
}
#[derive(Debug, Clone)]
pub struct Caption {
    attributes: Vec<CaptionAttributes>,
    body: Vec<crate::Node>,
}
impl kalosm_sample::Parse for Caption {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        CaptionAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</caption>")
            .map_output(|(attributes, body)| Caption { attributes, body })
    }
}
