use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum AreaAttributesName {
    #[parse(rename = " accesskey=")]
    Accesskey,
    #[parse(rename = " alt=")]
    Alt,
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
    #[parse(rename = " coords=")]
    Coords,
    #[parse(rename = " dir=")]
    Dir,
    #[parse(rename = " download=")]
    Download,
    #[parse(rename = " draggable=")]
    Draggable,
    #[parse(rename = " dropzone=")]
    Dropzone,
    #[parse(rename = " exportparts=")]
    Exportparts,
    #[parse(rename = " hidden=")]
    Hidden,
    #[parse(rename = " href=")]
    Href,
    #[parse(rename = " hreflang=")]
    Hreflang,
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
    #[parse(rename = " ping=")]
    Ping,
    #[parse(rename = " rel=")]
    Rel,
    #[parse(rename = " role=")]
    Role,
    #[parse(rename = " shape=")]
    Shape,
    #[parse(rename = " slot=")]
    Slot,
    #[parse(rename = " spellcheck=")]
    Spellcheck,
    #[parse(rename = " style=")]
    Style,
    #[parse(rename = " tabindex=")]
    Tabindex,
    #[parse(rename = " target=")]
    Target,
    #[parse(rename = " title=")]
    Title,
    #[parse(rename = " translate=")]
    Translate,
    #[parse(rename = " type=")]
    Type,
}
#[derive(Debug, Clone)]
pub enum AreaAttributes {
    Accesskey(String),
    Alt(String),
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
    Coords(String),
    Dir(crate::DValues),
    Download(String),
    Draggable(crate::BValues),
    Dropzone(String),
    Exportparts(String),
    Hidden(String),
    Href(String),
    Hreflang(String),
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
    Ping(String),
    Rel(String),
    Role(crate::RolesValues),
    Shape(crate::ShValues),
    Slot(String),
    Spellcheck(crate::BValues),
    Style(String),
    Tabindex(String),
    Target(crate::TargetValues),
    Title(String),
    Translate(crate::YValues),
    Type(String),
}
impl kalosm_sample::Parse for AreaAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        AreaAttributesName::new_parser()
            .then_lazy(|name| match name {
                AreaAttributesName::Accesskey => {
                    String::new_parser().map_output(Self::Accesskey).boxed()
                }
                AreaAttributesName::Alt => String::new_parser().map_output(Self::Alt).boxed(),
                AreaAttributesName::AriaActivedescendant => String::new_parser()
                    .map_output(Self::AriaActivedescendant)
                    .boxed(),
                AreaAttributesName::AriaAtomic => crate::BValues::new_parser()
                    .map_output(Self::AriaAtomic)
                    .boxed(),
                AreaAttributesName::AriaAutocomplete => crate::AutocompleteValues::new_parser()
                    .map_output(Self::AriaAutocomplete)
                    .boxed(),
                AreaAttributesName::AriaBusy => crate::BValues::new_parser()
                    .map_output(Self::AriaBusy)
                    .boxed(),
                AreaAttributesName::AriaChecked => crate::TristateValues::new_parser()
                    .map_output(Self::AriaChecked)
                    .boxed(),
                AreaAttributesName::AriaColcount => {
                    String::new_parser().map_output(Self::AriaColcount).boxed()
                }
                AreaAttributesName::AriaColindex => {
                    String::new_parser().map_output(Self::AriaColindex).boxed()
                }
                AreaAttributesName::AriaColspan => {
                    String::new_parser().map_output(Self::AriaColspan).boxed()
                }
                AreaAttributesName::AriaControls => {
                    String::new_parser().map_output(Self::AriaControls).boxed()
                }
                AreaAttributesName::AriaCurrent => crate::CurrentValues::new_parser()
                    .map_output(Self::AriaCurrent)
                    .boxed(),
                AreaAttributesName::AriaDescribedby => String::new_parser()
                    .map_output(Self::AriaDescribedby)
                    .boxed(),
                AreaAttributesName::AriaDetails => {
                    String::new_parser().map_output(Self::AriaDetails).boxed()
                }
                AreaAttributesName::AriaDisabled => crate::BValues::new_parser()
                    .map_output(Self::AriaDisabled)
                    .boxed(),
                AreaAttributesName::AriaDropeffect => crate::DropeffectValues::new_parser()
                    .map_output(Self::AriaDropeffect)
                    .boxed(),
                AreaAttributesName::AriaErrormessage => String::new_parser()
                    .map_output(Self::AriaErrormessage)
                    .boxed(),
                AreaAttributesName::AriaExpanded => crate::UValues::new_parser()
                    .map_output(Self::AriaExpanded)
                    .boxed(),
                AreaAttributesName::AriaFlowto => {
                    String::new_parser().map_output(Self::AriaFlowto).boxed()
                }
                AreaAttributesName::AriaGrabbed => crate::UValues::new_parser()
                    .map_output(Self::AriaGrabbed)
                    .boxed(),
                AreaAttributesName::AriaHaspopup => crate::HaspopupValues::new_parser()
                    .map_output(Self::AriaHaspopup)
                    .boxed(),
                AreaAttributesName::AriaHidden => crate::BValues::new_parser()
                    .map_output(Self::AriaHidden)
                    .boxed(),
                AreaAttributesName::AriaInvalid => crate::InvalidValues::new_parser()
                    .map_output(Self::AriaInvalid)
                    .boxed(),
                AreaAttributesName::AriaKeyshortcuts => String::new_parser()
                    .map_output(Self::AriaKeyshortcuts)
                    .boxed(),
                AreaAttributesName::AriaLabel => {
                    String::new_parser().map_output(Self::AriaLabel).boxed()
                }
                AreaAttributesName::AriaLabelledby => String::new_parser()
                    .map_output(Self::AriaLabelledby)
                    .boxed(),
                AreaAttributesName::AriaLevel => {
                    String::new_parser().map_output(Self::AriaLevel).boxed()
                }
                AreaAttributesName::AriaLive => crate::LiveValues::new_parser()
                    .map_output(Self::AriaLive)
                    .boxed(),
                AreaAttributesName::AriaModal => crate::BValues::new_parser()
                    .map_output(Self::AriaModal)
                    .boxed(),
                AreaAttributesName::AriaMultiline => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiline)
                    .boxed(),
                AreaAttributesName::AriaMultiselectable => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiselectable)
                    .boxed(),
                AreaAttributesName::AriaOrientation => crate::OrientationValues::new_parser()
                    .map_output(Self::AriaOrientation)
                    .boxed(),
                AreaAttributesName::AriaOwns => {
                    String::new_parser().map_output(Self::AriaOwns).boxed()
                }
                AreaAttributesName::AriaPlaceholder => String::new_parser()
                    .map_output(Self::AriaPlaceholder)
                    .boxed(),
                AreaAttributesName::AriaPosinset => {
                    String::new_parser().map_output(Self::AriaPosinset).boxed()
                }
                AreaAttributesName::AriaPressed => crate::TristateValues::new_parser()
                    .map_output(Self::AriaPressed)
                    .boxed(),
                AreaAttributesName::AriaReadonly => crate::BValues::new_parser()
                    .map_output(Self::AriaReadonly)
                    .boxed(),
                AreaAttributesName::AriaRelevant => crate::RelevantValues::new_parser()
                    .map_output(Self::AriaRelevant)
                    .boxed(),
                AreaAttributesName::AriaRequired => crate::BValues::new_parser()
                    .map_output(Self::AriaRequired)
                    .boxed(),
                AreaAttributesName::AriaRoledescription => String::new_parser()
                    .map_output(Self::AriaRoledescription)
                    .boxed(),
                AreaAttributesName::AriaRowcount => {
                    String::new_parser().map_output(Self::AriaRowcount).boxed()
                }
                AreaAttributesName::AriaRowindex => {
                    String::new_parser().map_output(Self::AriaRowindex).boxed()
                }
                AreaAttributesName::AriaRowspan => {
                    String::new_parser().map_output(Self::AriaRowspan).boxed()
                }
                AreaAttributesName::AriaSelected => crate::UValues::new_parser()
                    .map_output(Self::AriaSelected)
                    .boxed(),
                AreaAttributesName::AriaSetsize => {
                    String::new_parser().map_output(Self::AriaSetsize).boxed()
                }
                AreaAttributesName::AriaSort => crate::SortValues::new_parser()
                    .map_output(Self::AriaSort)
                    .boxed(),
                AreaAttributesName::AriaValuemax => {
                    String::new_parser().map_output(Self::AriaValuemax).boxed()
                }
                AreaAttributesName::AriaValuemin => {
                    String::new_parser().map_output(Self::AriaValuemin).boxed()
                }
                AreaAttributesName::AriaValuenow => {
                    String::new_parser().map_output(Self::AriaValuenow).boxed()
                }
                AreaAttributesName::AriaValuetext => {
                    String::new_parser().map_output(Self::AriaValuetext).boxed()
                }
                AreaAttributesName::Autocapitalize => String::new_parser()
                    .map_output(Self::Autocapitalize)
                    .boxed(),
                AreaAttributesName::Class => String::new_parser().map_output(Self::Class).boxed(),
                AreaAttributesName::Contenteditable => String::new_parser()
                    .map_output(Self::Contenteditable)
                    .boxed(),
                AreaAttributesName::Contextmenu => {
                    String::new_parser().map_output(Self::Contextmenu).boxed()
                }
                AreaAttributesName::Coords => String::new_parser().map_output(Self::Coords).boxed(),
                AreaAttributesName::Dir => {
                    crate::DValues::new_parser().map_output(Self::Dir).boxed()
                }
                AreaAttributesName::Download => {
                    String::new_parser().map_output(Self::Download).boxed()
                }
                AreaAttributesName::Draggable => crate::BValues::new_parser()
                    .map_output(Self::Draggable)
                    .boxed(),
                AreaAttributesName::Dropzone => {
                    String::new_parser().map_output(Self::Dropzone).boxed()
                }
                AreaAttributesName::Exportparts => {
                    String::new_parser().map_output(Self::Exportparts).boxed()
                }
                AreaAttributesName::Hidden => String::new_parser().map_output(Self::Hidden).boxed(),
                AreaAttributesName::Href => String::new_parser().map_output(Self::Href).boxed(),
                AreaAttributesName::Hreflang => {
                    String::new_parser().map_output(Self::Hreflang).boxed()
                }
                AreaAttributesName::Id => String::new_parser().map_output(Self::Id).boxed(),
                AreaAttributesName::Inputmode => {
                    String::new_parser().map_output(Self::Inputmode).boxed()
                }
                AreaAttributesName::Is => String::new_parser().map_output(Self::Is).boxed(),
                AreaAttributesName::Itemid => String::new_parser().map_output(Self::Itemid).boxed(),
                AreaAttributesName::Itemprop => {
                    String::new_parser().map_output(Self::Itemprop).boxed()
                }
                AreaAttributesName::Itemref => {
                    String::new_parser().map_output(Self::Itemref).boxed()
                }
                AreaAttributesName::Itemscope => {
                    String::new_parser().map_output(Self::Itemscope).boxed()
                }
                AreaAttributesName::Itemtype => {
                    String::new_parser().map_output(Self::Itemtype).boxed()
                }
                AreaAttributesName::Lang => String::new_parser().map_output(Self::Lang).boxed(),
                AreaAttributesName::Onabort => {
                    String::new_parser().map_output(Self::Onabort).boxed()
                }
                AreaAttributesName::Onblur => String::new_parser().map_output(Self::Onblur).boxed(),
                AreaAttributesName::Oncanplay => {
                    String::new_parser().map_output(Self::Oncanplay).boxed()
                }
                AreaAttributesName::Oncanplaythrough => String::new_parser()
                    .map_output(Self::Oncanplaythrough)
                    .boxed(),
                AreaAttributesName::Onchange => {
                    String::new_parser().map_output(Self::Onchange).boxed()
                }
                AreaAttributesName::Onclick => {
                    String::new_parser().map_output(Self::Onclick).boxed()
                }
                AreaAttributesName::Oncontextmenu => {
                    String::new_parser().map_output(Self::Oncontextmenu).boxed()
                }
                AreaAttributesName::Ondblclick => {
                    String::new_parser().map_output(Self::Ondblclick).boxed()
                }
                AreaAttributesName::Ondrag => String::new_parser().map_output(Self::Ondrag).boxed(),
                AreaAttributesName::Ondragend => {
                    String::new_parser().map_output(Self::Ondragend).boxed()
                }
                AreaAttributesName::Ondragenter => {
                    String::new_parser().map_output(Self::Ondragenter).boxed()
                }
                AreaAttributesName::Ondragleave => {
                    String::new_parser().map_output(Self::Ondragleave).boxed()
                }
                AreaAttributesName::Ondragover => {
                    String::new_parser().map_output(Self::Ondragover).boxed()
                }
                AreaAttributesName::Ondragstart => {
                    String::new_parser().map_output(Self::Ondragstart).boxed()
                }
                AreaAttributesName::Ondrop => String::new_parser().map_output(Self::Ondrop).boxed(),
                AreaAttributesName::Ondurationchange => String::new_parser()
                    .map_output(Self::Ondurationchange)
                    .boxed(),
                AreaAttributesName::Onemptied => {
                    String::new_parser().map_output(Self::Onemptied).boxed()
                }
                AreaAttributesName::Onended => {
                    String::new_parser().map_output(Self::Onended).boxed()
                }
                AreaAttributesName::Onerror => {
                    String::new_parser().map_output(Self::Onerror).boxed()
                }
                AreaAttributesName::Onfocus => {
                    String::new_parser().map_output(Self::Onfocus).boxed()
                }
                AreaAttributesName::Onformchange => {
                    String::new_parser().map_output(Self::Onformchange).boxed()
                }
                AreaAttributesName::Onforminput => {
                    String::new_parser().map_output(Self::Onforminput).boxed()
                }
                AreaAttributesName::Oninput => {
                    String::new_parser().map_output(Self::Oninput).boxed()
                }
                AreaAttributesName::Oninvalid => {
                    String::new_parser().map_output(Self::Oninvalid).boxed()
                }
                AreaAttributesName::Onkeydown => {
                    String::new_parser().map_output(Self::Onkeydown).boxed()
                }
                AreaAttributesName::Onkeypress => {
                    String::new_parser().map_output(Self::Onkeypress).boxed()
                }
                AreaAttributesName::Onkeyup => {
                    String::new_parser().map_output(Self::Onkeyup).boxed()
                }
                AreaAttributesName::Onload => String::new_parser().map_output(Self::Onload).boxed(),
                AreaAttributesName::Onloadeddata => {
                    String::new_parser().map_output(Self::Onloadeddata).boxed()
                }
                AreaAttributesName::Onloadedmetadata => String::new_parser()
                    .map_output(Self::Onloadedmetadata)
                    .boxed(),
                AreaAttributesName::Onloadstart => {
                    String::new_parser().map_output(Self::Onloadstart).boxed()
                }
                AreaAttributesName::Onmousedown => {
                    String::new_parser().map_output(Self::Onmousedown).boxed()
                }
                AreaAttributesName::Onmouseenter => {
                    String::new_parser().map_output(Self::Onmouseenter).boxed()
                }
                AreaAttributesName::Onmouseleave => {
                    String::new_parser().map_output(Self::Onmouseleave).boxed()
                }
                AreaAttributesName::Onmousemove => {
                    String::new_parser().map_output(Self::Onmousemove).boxed()
                }
                AreaAttributesName::Onmouseout => {
                    String::new_parser().map_output(Self::Onmouseout).boxed()
                }
                AreaAttributesName::Onmouseover => {
                    String::new_parser().map_output(Self::Onmouseover).boxed()
                }
                AreaAttributesName::Onmouseup => {
                    String::new_parser().map_output(Self::Onmouseup).boxed()
                }
                AreaAttributesName::Onmousewheel => {
                    String::new_parser().map_output(Self::Onmousewheel).boxed()
                }
                AreaAttributesName::Onpause => {
                    String::new_parser().map_output(Self::Onpause).boxed()
                }
                AreaAttributesName::Onplay => String::new_parser().map_output(Self::Onplay).boxed(),
                AreaAttributesName::Onplaying => {
                    String::new_parser().map_output(Self::Onplaying).boxed()
                }
                AreaAttributesName::Onpointercancel => String::new_parser()
                    .map_output(Self::Onpointercancel)
                    .boxed(),
                AreaAttributesName::Onpointerdown => {
                    String::new_parser().map_output(Self::Onpointerdown).boxed()
                }
                AreaAttributesName::Onpointerenter => String::new_parser()
                    .map_output(Self::Onpointerenter)
                    .boxed(),
                AreaAttributesName::Onpointerleave => String::new_parser()
                    .map_output(Self::Onpointerleave)
                    .boxed(),
                AreaAttributesName::Onpointerlockchange => String::new_parser()
                    .map_output(Self::Onpointerlockchange)
                    .boxed(),
                AreaAttributesName::Onpointerlockerror => String::new_parser()
                    .map_output(Self::Onpointerlockerror)
                    .boxed(),
                AreaAttributesName::Onpointermove => {
                    String::new_parser().map_output(Self::Onpointermove).boxed()
                }
                AreaAttributesName::Onpointerout => {
                    String::new_parser().map_output(Self::Onpointerout).boxed()
                }
                AreaAttributesName::Onpointerover => {
                    String::new_parser().map_output(Self::Onpointerover).boxed()
                }
                AreaAttributesName::Onpointerup => {
                    String::new_parser().map_output(Self::Onpointerup).boxed()
                }
                AreaAttributesName::Onprogress => {
                    String::new_parser().map_output(Self::Onprogress).boxed()
                }
                AreaAttributesName::Onratechange => {
                    String::new_parser().map_output(Self::Onratechange).boxed()
                }
                AreaAttributesName::Onreadystatechange => String::new_parser()
                    .map_output(Self::Onreadystatechange)
                    .boxed(),
                AreaAttributesName::Onreset => {
                    String::new_parser().map_output(Self::Onreset).boxed()
                }
                AreaAttributesName::Onresize => {
                    String::new_parser().map_output(Self::Onresize).boxed()
                }
                AreaAttributesName::Onscroll => {
                    String::new_parser().map_output(Self::Onscroll).boxed()
                }
                AreaAttributesName::Onseeked => {
                    String::new_parser().map_output(Self::Onseeked).boxed()
                }
                AreaAttributesName::Onseeking => {
                    String::new_parser().map_output(Self::Onseeking).boxed()
                }
                AreaAttributesName::Onselect => {
                    String::new_parser().map_output(Self::Onselect).boxed()
                }
                AreaAttributesName::Onshow => String::new_parser().map_output(Self::Onshow).boxed(),
                AreaAttributesName::Onstalled => {
                    String::new_parser().map_output(Self::Onstalled).boxed()
                }
                AreaAttributesName::Onsubmit => {
                    String::new_parser().map_output(Self::Onsubmit).boxed()
                }
                AreaAttributesName::Onsuspend => {
                    String::new_parser().map_output(Self::Onsuspend).boxed()
                }
                AreaAttributesName::Ontimeupdate => {
                    String::new_parser().map_output(Self::Ontimeupdate).boxed()
                }
                AreaAttributesName::Onvolumechange => String::new_parser()
                    .map_output(Self::Onvolumechange)
                    .boxed(),
                AreaAttributesName::Onwaiting => {
                    String::new_parser().map_output(Self::Onwaiting).boxed()
                }
                AreaAttributesName::Part => String::new_parser().map_output(Self::Part).boxed(),
                AreaAttributesName::Ping => String::new_parser().map_output(Self::Ping).boxed(),
                AreaAttributesName::Rel => String::new_parser().map_output(Self::Rel).boxed(),
                AreaAttributesName::Role => crate::RolesValues::new_parser()
                    .map_output(Self::Role)
                    .boxed(),
                AreaAttributesName::Shape => crate::ShValues::new_parser()
                    .map_output(Self::Shape)
                    .boxed(),
                AreaAttributesName::Slot => String::new_parser().map_output(Self::Slot).boxed(),
                AreaAttributesName::Spellcheck => crate::BValues::new_parser()
                    .map_output(Self::Spellcheck)
                    .boxed(),
                AreaAttributesName::Style => String::new_parser().map_output(Self::Style).boxed(),
                AreaAttributesName::Tabindex => {
                    String::new_parser().map_output(Self::Tabindex).boxed()
                }
                AreaAttributesName::Target => crate::TargetValues::new_parser()
                    .map_output(Self::Target)
                    .boxed(),
                AreaAttributesName::Title => String::new_parser().map_output(Self::Title).boxed(),
                AreaAttributesName::Translate => crate::YValues::new_parser()
                    .map_output(Self::Translate)
                    .boxed(),
                AreaAttributesName::Type => String::new_parser().map_output(Self::Type).boxed(),
            })
            .map_output(|(_, attribute)| attribute)
    }
}
#[derive(Debug, Clone)]
pub struct Area {
    attributes: Vec<AreaAttributes>,
}
impl kalosm_sample::Parse for Area {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        AreaAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Area { attributes })
    }
}
