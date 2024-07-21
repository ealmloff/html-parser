use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ImgAttributesName {
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
    #[parse(rename = " crossorigin=")]
    Crossorigin,
    #[parse(rename = " decoding=")]
    Decoding,
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
    #[parse(rename = " importance=")]
    Importance,
    #[parse(rename = " inputmode=")]
    Inputmode,
    #[parse(rename = " intrinsicsize=")]
    Intrinsicsize,
    #[parse(rename = " is=")]
    Is,
    #[parse(rename = " ismap=")]
    Ismap,
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
    #[parse(rename = " loading=")]
    Loading,
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
    #[parse(rename = " referrerpolicy=")]
    Referrerpolicy,
    #[parse(rename = " role=")]
    Role,
    #[parse(rename = " sizes=")]
    Sizes,
    #[parse(rename = " slot=")]
    Slot,
    #[parse(rename = " spellcheck=")]
    Spellcheck,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " srcset=")]
    Srcset,
    #[parse(rename = " style=")]
    Style,
    #[parse(rename = " tabindex=")]
    Tabindex,
    #[parse(rename = " title=")]
    Title,
    #[parse(rename = " translate=")]
    Translate,
    #[parse(rename = " usemap=")]
    Usemap,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum ImgAttributes {
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
    Crossorigin(crate::XoValues),
    Decoding(crate::DecodingValues),
    Dir(crate::DValues),
    Draggable(crate::BValues),
    Dropzone(String),
    Exportparts(String),
    Height(String),
    Hidden(String),
    Id(String),
    Importance(String),
    Inputmode(String),
    Intrinsicsize(String),
    Is(String),
    Ismap(String),
    Itemid(String),
    Itemprop(String),
    Itemref(String),
    Itemscope(String),
    Itemtype(String),
    Lang(String),
    Loading(crate::LoadingValues),
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
    Referrerpolicy(crate::ReferrerpolicyValues),
    Role(crate::RolesValues),
    Sizes(String),
    Slot(String),
    Spellcheck(crate::BValues),
    Src(String),
    Srcset(String),
    Style(String),
    Tabindex(String),
    Title(String),
    Translate(crate::YValues),
    Usemap(String),
    Width(String),
}
impl kalosm_sample::Parse for ImgAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ImgAttributesName::new_parser()
            .then_lazy(|name| match name {
                ImgAttributesName::Accesskey => {
                    String::new_parser().map_output(Self::Accesskey).boxed()
                }
                ImgAttributesName::Alt => String::new_parser().map_output(Self::Alt).boxed(),
                ImgAttributesName::AriaActivedescendant => String::new_parser()
                    .map_output(Self::AriaActivedescendant)
                    .boxed(),
                ImgAttributesName::AriaAtomic => crate::BValues::new_parser()
                    .map_output(Self::AriaAtomic)
                    .boxed(),
                ImgAttributesName::AriaAutocomplete => crate::AutocompleteValues::new_parser()
                    .map_output(Self::AriaAutocomplete)
                    .boxed(),
                ImgAttributesName::AriaBusy => crate::BValues::new_parser()
                    .map_output(Self::AriaBusy)
                    .boxed(),
                ImgAttributesName::AriaChecked => crate::TristateValues::new_parser()
                    .map_output(Self::AriaChecked)
                    .boxed(),
                ImgAttributesName::AriaColcount => {
                    String::new_parser().map_output(Self::AriaColcount).boxed()
                }
                ImgAttributesName::AriaColindex => {
                    String::new_parser().map_output(Self::AriaColindex).boxed()
                }
                ImgAttributesName::AriaColspan => {
                    String::new_parser().map_output(Self::AriaColspan).boxed()
                }
                ImgAttributesName::AriaControls => {
                    String::new_parser().map_output(Self::AriaControls).boxed()
                }
                ImgAttributesName::AriaCurrent => crate::CurrentValues::new_parser()
                    .map_output(Self::AriaCurrent)
                    .boxed(),
                ImgAttributesName::AriaDescribedby => String::new_parser()
                    .map_output(Self::AriaDescribedby)
                    .boxed(),
                ImgAttributesName::AriaDetails => {
                    String::new_parser().map_output(Self::AriaDetails).boxed()
                }
                ImgAttributesName::AriaDisabled => crate::BValues::new_parser()
                    .map_output(Self::AriaDisabled)
                    .boxed(),
                ImgAttributesName::AriaDropeffect => crate::DropeffectValues::new_parser()
                    .map_output(Self::AriaDropeffect)
                    .boxed(),
                ImgAttributesName::AriaErrormessage => String::new_parser()
                    .map_output(Self::AriaErrormessage)
                    .boxed(),
                ImgAttributesName::AriaExpanded => crate::UValues::new_parser()
                    .map_output(Self::AriaExpanded)
                    .boxed(),
                ImgAttributesName::AriaFlowto => {
                    String::new_parser().map_output(Self::AriaFlowto).boxed()
                }
                ImgAttributesName::AriaGrabbed => crate::UValues::new_parser()
                    .map_output(Self::AriaGrabbed)
                    .boxed(),
                ImgAttributesName::AriaHaspopup => crate::HaspopupValues::new_parser()
                    .map_output(Self::AriaHaspopup)
                    .boxed(),
                ImgAttributesName::AriaHidden => crate::BValues::new_parser()
                    .map_output(Self::AriaHidden)
                    .boxed(),
                ImgAttributesName::AriaInvalid => crate::InvalidValues::new_parser()
                    .map_output(Self::AriaInvalid)
                    .boxed(),
                ImgAttributesName::AriaKeyshortcuts => String::new_parser()
                    .map_output(Self::AriaKeyshortcuts)
                    .boxed(),
                ImgAttributesName::AriaLabel => {
                    String::new_parser().map_output(Self::AriaLabel).boxed()
                }
                ImgAttributesName::AriaLabelledby => String::new_parser()
                    .map_output(Self::AriaLabelledby)
                    .boxed(),
                ImgAttributesName::AriaLevel => {
                    String::new_parser().map_output(Self::AriaLevel).boxed()
                }
                ImgAttributesName::AriaLive => crate::LiveValues::new_parser()
                    .map_output(Self::AriaLive)
                    .boxed(),
                ImgAttributesName::AriaModal => crate::BValues::new_parser()
                    .map_output(Self::AriaModal)
                    .boxed(),
                ImgAttributesName::AriaMultiline => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiline)
                    .boxed(),
                ImgAttributesName::AriaMultiselectable => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiselectable)
                    .boxed(),
                ImgAttributesName::AriaOrientation => crate::OrientationValues::new_parser()
                    .map_output(Self::AriaOrientation)
                    .boxed(),
                ImgAttributesName::AriaOwns => {
                    String::new_parser().map_output(Self::AriaOwns).boxed()
                }
                ImgAttributesName::AriaPlaceholder => String::new_parser()
                    .map_output(Self::AriaPlaceholder)
                    .boxed(),
                ImgAttributesName::AriaPosinset => {
                    String::new_parser().map_output(Self::AriaPosinset).boxed()
                }
                ImgAttributesName::AriaPressed => crate::TristateValues::new_parser()
                    .map_output(Self::AriaPressed)
                    .boxed(),
                ImgAttributesName::AriaReadonly => crate::BValues::new_parser()
                    .map_output(Self::AriaReadonly)
                    .boxed(),
                ImgAttributesName::AriaRelevant => crate::RelevantValues::new_parser()
                    .map_output(Self::AriaRelevant)
                    .boxed(),
                ImgAttributesName::AriaRequired => crate::BValues::new_parser()
                    .map_output(Self::AriaRequired)
                    .boxed(),
                ImgAttributesName::AriaRoledescription => String::new_parser()
                    .map_output(Self::AriaRoledescription)
                    .boxed(),
                ImgAttributesName::AriaRowcount => {
                    String::new_parser().map_output(Self::AriaRowcount).boxed()
                }
                ImgAttributesName::AriaRowindex => {
                    String::new_parser().map_output(Self::AriaRowindex).boxed()
                }
                ImgAttributesName::AriaRowspan => {
                    String::new_parser().map_output(Self::AriaRowspan).boxed()
                }
                ImgAttributesName::AriaSelected => crate::UValues::new_parser()
                    .map_output(Self::AriaSelected)
                    .boxed(),
                ImgAttributesName::AriaSetsize => {
                    String::new_parser().map_output(Self::AriaSetsize).boxed()
                }
                ImgAttributesName::AriaSort => crate::SortValues::new_parser()
                    .map_output(Self::AriaSort)
                    .boxed(),
                ImgAttributesName::AriaValuemax => {
                    String::new_parser().map_output(Self::AriaValuemax).boxed()
                }
                ImgAttributesName::AriaValuemin => {
                    String::new_parser().map_output(Self::AriaValuemin).boxed()
                }
                ImgAttributesName::AriaValuenow => {
                    String::new_parser().map_output(Self::AriaValuenow).boxed()
                }
                ImgAttributesName::AriaValuetext => {
                    String::new_parser().map_output(Self::AriaValuetext).boxed()
                }
                ImgAttributesName::Autocapitalize => String::new_parser()
                    .map_output(Self::Autocapitalize)
                    .boxed(),
                ImgAttributesName::Class => String::new_parser().map_output(Self::Class).boxed(),
                ImgAttributesName::Contenteditable => String::new_parser()
                    .map_output(Self::Contenteditable)
                    .boxed(),
                ImgAttributesName::Contextmenu => {
                    String::new_parser().map_output(Self::Contextmenu).boxed()
                }
                ImgAttributesName::Crossorigin => crate::XoValues::new_parser()
                    .map_output(Self::Crossorigin)
                    .boxed(),
                ImgAttributesName::Decoding => crate::DecodingValues::new_parser()
                    .map_output(Self::Decoding)
                    .boxed(),
                ImgAttributesName::Dir => {
                    crate::DValues::new_parser().map_output(Self::Dir).boxed()
                }
                ImgAttributesName::Draggable => crate::BValues::new_parser()
                    .map_output(Self::Draggable)
                    .boxed(),
                ImgAttributesName::Dropzone => {
                    String::new_parser().map_output(Self::Dropzone).boxed()
                }
                ImgAttributesName::Exportparts => {
                    String::new_parser().map_output(Self::Exportparts).boxed()
                }
                ImgAttributesName::Height => String::new_parser().map_output(Self::Height).boxed(),
                ImgAttributesName::Hidden => String::new_parser().map_output(Self::Hidden).boxed(),
                ImgAttributesName::Id => String::new_parser().map_output(Self::Id).boxed(),
                ImgAttributesName::Importance => {
                    String::new_parser().map_output(Self::Importance).boxed()
                }
                ImgAttributesName::Inputmode => {
                    String::new_parser().map_output(Self::Inputmode).boxed()
                }
                ImgAttributesName::Intrinsicsize => {
                    String::new_parser().map_output(Self::Intrinsicsize).boxed()
                }
                ImgAttributesName::Is => String::new_parser().map_output(Self::Is).boxed(),
                ImgAttributesName::Ismap => String::new_parser().map_output(Self::Ismap).boxed(),
                ImgAttributesName::Itemid => String::new_parser().map_output(Self::Itemid).boxed(),
                ImgAttributesName::Itemprop => {
                    String::new_parser().map_output(Self::Itemprop).boxed()
                }
                ImgAttributesName::Itemref => {
                    String::new_parser().map_output(Self::Itemref).boxed()
                }
                ImgAttributesName::Itemscope => {
                    String::new_parser().map_output(Self::Itemscope).boxed()
                }
                ImgAttributesName::Itemtype => {
                    String::new_parser().map_output(Self::Itemtype).boxed()
                }
                ImgAttributesName::Lang => String::new_parser().map_output(Self::Lang).boxed(),
                ImgAttributesName::Loading => crate::LoadingValues::new_parser()
                    .map_output(Self::Loading)
                    .boxed(),
                ImgAttributesName::Onabort => {
                    String::new_parser().map_output(Self::Onabort).boxed()
                }
                ImgAttributesName::Onblur => String::new_parser().map_output(Self::Onblur).boxed(),
                ImgAttributesName::Oncanplay => {
                    String::new_parser().map_output(Self::Oncanplay).boxed()
                }
                ImgAttributesName::Oncanplaythrough => String::new_parser()
                    .map_output(Self::Oncanplaythrough)
                    .boxed(),
                ImgAttributesName::Onchange => {
                    String::new_parser().map_output(Self::Onchange).boxed()
                }
                ImgAttributesName::Onclick => {
                    String::new_parser().map_output(Self::Onclick).boxed()
                }
                ImgAttributesName::Oncontextmenu => {
                    String::new_parser().map_output(Self::Oncontextmenu).boxed()
                }
                ImgAttributesName::Ondblclick => {
                    String::new_parser().map_output(Self::Ondblclick).boxed()
                }
                ImgAttributesName::Ondrag => String::new_parser().map_output(Self::Ondrag).boxed(),
                ImgAttributesName::Ondragend => {
                    String::new_parser().map_output(Self::Ondragend).boxed()
                }
                ImgAttributesName::Ondragenter => {
                    String::new_parser().map_output(Self::Ondragenter).boxed()
                }
                ImgAttributesName::Ondragleave => {
                    String::new_parser().map_output(Self::Ondragleave).boxed()
                }
                ImgAttributesName::Ondragover => {
                    String::new_parser().map_output(Self::Ondragover).boxed()
                }
                ImgAttributesName::Ondragstart => {
                    String::new_parser().map_output(Self::Ondragstart).boxed()
                }
                ImgAttributesName::Ondrop => String::new_parser().map_output(Self::Ondrop).boxed(),
                ImgAttributesName::Ondurationchange => String::new_parser()
                    .map_output(Self::Ondurationchange)
                    .boxed(),
                ImgAttributesName::Onemptied => {
                    String::new_parser().map_output(Self::Onemptied).boxed()
                }
                ImgAttributesName::Onended => {
                    String::new_parser().map_output(Self::Onended).boxed()
                }
                ImgAttributesName::Onerror => {
                    String::new_parser().map_output(Self::Onerror).boxed()
                }
                ImgAttributesName::Onfocus => {
                    String::new_parser().map_output(Self::Onfocus).boxed()
                }
                ImgAttributesName::Onformchange => {
                    String::new_parser().map_output(Self::Onformchange).boxed()
                }
                ImgAttributesName::Onforminput => {
                    String::new_parser().map_output(Self::Onforminput).boxed()
                }
                ImgAttributesName::Oninput => {
                    String::new_parser().map_output(Self::Oninput).boxed()
                }
                ImgAttributesName::Oninvalid => {
                    String::new_parser().map_output(Self::Oninvalid).boxed()
                }
                ImgAttributesName::Onkeydown => {
                    String::new_parser().map_output(Self::Onkeydown).boxed()
                }
                ImgAttributesName::Onkeypress => {
                    String::new_parser().map_output(Self::Onkeypress).boxed()
                }
                ImgAttributesName::Onkeyup => {
                    String::new_parser().map_output(Self::Onkeyup).boxed()
                }
                ImgAttributesName::Onload => String::new_parser().map_output(Self::Onload).boxed(),
                ImgAttributesName::Onloadeddata => {
                    String::new_parser().map_output(Self::Onloadeddata).boxed()
                }
                ImgAttributesName::Onloadedmetadata => String::new_parser()
                    .map_output(Self::Onloadedmetadata)
                    .boxed(),
                ImgAttributesName::Onloadstart => {
                    String::new_parser().map_output(Self::Onloadstart).boxed()
                }
                ImgAttributesName::Onmousedown => {
                    String::new_parser().map_output(Self::Onmousedown).boxed()
                }
                ImgAttributesName::Onmouseenter => {
                    String::new_parser().map_output(Self::Onmouseenter).boxed()
                }
                ImgAttributesName::Onmouseleave => {
                    String::new_parser().map_output(Self::Onmouseleave).boxed()
                }
                ImgAttributesName::Onmousemove => {
                    String::new_parser().map_output(Self::Onmousemove).boxed()
                }
                ImgAttributesName::Onmouseout => {
                    String::new_parser().map_output(Self::Onmouseout).boxed()
                }
                ImgAttributesName::Onmouseover => {
                    String::new_parser().map_output(Self::Onmouseover).boxed()
                }
                ImgAttributesName::Onmouseup => {
                    String::new_parser().map_output(Self::Onmouseup).boxed()
                }
                ImgAttributesName::Onmousewheel => {
                    String::new_parser().map_output(Self::Onmousewheel).boxed()
                }
                ImgAttributesName::Onpause => {
                    String::new_parser().map_output(Self::Onpause).boxed()
                }
                ImgAttributesName::Onplay => String::new_parser().map_output(Self::Onplay).boxed(),
                ImgAttributesName::Onplaying => {
                    String::new_parser().map_output(Self::Onplaying).boxed()
                }
                ImgAttributesName::Onpointercancel => String::new_parser()
                    .map_output(Self::Onpointercancel)
                    .boxed(),
                ImgAttributesName::Onpointerdown => {
                    String::new_parser().map_output(Self::Onpointerdown).boxed()
                }
                ImgAttributesName::Onpointerenter => String::new_parser()
                    .map_output(Self::Onpointerenter)
                    .boxed(),
                ImgAttributesName::Onpointerleave => String::new_parser()
                    .map_output(Self::Onpointerleave)
                    .boxed(),
                ImgAttributesName::Onpointerlockchange => String::new_parser()
                    .map_output(Self::Onpointerlockchange)
                    .boxed(),
                ImgAttributesName::Onpointerlockerror => String::new_parser()
                    .map_output(Self::Onpointerlockerror)
                    .boxed(),
                ImgAttributesName::Onpointermove => {
                    String::new_parser().map_output(Self::Onpointermove).boxed()
                }
                ImgAttributesName::Onpointerout => {
                    String::new_parser().map_output(Self::Onpointerout).boxed()
                }
                ImgAttributesName::Onpointerover => {
                    String::new_parser().map_output(Self::Onpointerover).boxed()
                }
                ImgAttributesName::Onpointerup => {
                    String::new_parser().map_output(Self::Onpointerup).boxed()
                }
                ImgAttributesName::Onprogress => {
                    String::new_parser().map_output(Self::Onprogress).boxed()
                }
                ImgAttributesName::Onratechange => {
                    String::new_parser().map_output(Self::Onratechange).boxed()
                }
                ImgAttributesName::Onreadystatechange => String::new_parser()
                    .map_output(Self::Onreadystatechange)
                    .boxed(),
                ImgAttributesName::Onreset => {
                    String::new_parser().map_output(Self::Onreset).boxed()
                }
                ImgAttributesName::Onresize => {
                    String::new_parser().map_output(Self::Onresize).boxed()
                }
                ImgAttributesName::Onscroll => {
                    String::new_parser().map_output(Self::Onscroll).boxed()
                }
                ImgAttributesName::Onseeked => {
                    String::new_parser().map_output(Self::Onseeked).boxed()
                }
                ImgAttributesName::Onseeking => {
                    String::new_parser().map_output(Self::Onseeking).boxed()
                }
                ImgAttributesName::Onselect => {
                    String::new_parser().map_output(Self::Onselect).boxed()
                }
                ImgAttributesName::Onshow => String::new_parser().map_output(Self::Onshow).boxed(),
                ImgAttributesName::Onstalled => {
                    String::new_parser().map_output(Self::Onstalled).boxed()
                }
                ImgAttributesName::Onsubmit => {
                    String::new_parser().map_output(Self::Onsubmit).boxed()
                }
                ImgAttributesName::Onsuspend => {
                    String::new_parser().map_output(Self::Onsuspend).boxed()
                }
                ImgAttributesName::Ontimeupdate => {
                    String::new_parser().map_output(Self::Ontimeupdate).boxed()
                }
                ImgAttributesName::Onvolumechange => String::new_parser()
                    .map_output(Self::Onvolumechange)
                    .boxed(),
                ImgAttributesName::Onwaiting => {
                    String::new_parser().map_output(Self::Onwaiting).boxed()
                }
                ImgAttributesName::Part => String::new_parser().map_output(Self::Part).boxed(),
                ImgAttributesName::Referrerpolicy => crate::ReferrerpolicyValues::new_parser()
                    .map_output(Self::Referrerpolicy)
                    .boxed(),
                ImgAttributesName::Role => crate::RolesValues::new_parser()
                    .map_output(Self::Role)
                    .boxed(),
                ImgAttributesName::Sizes => String::new_parser().map_output(Self::Sizes).boxed(),
                ImgAttributesName::Slot => String::new_parser().map_output(Self::Slot).boxed(),
                ImgAttributesName::Spellcheck => crate::BValues::new_parser()
                    .map_output(Self::Spellcheck)
                    .boxed(),
                ImgAttributesName::Src => String::new_parser().map_output(Self::Src).boxed(),
                ImgAttributesName::Srcset => String::new_parser().map_output(Self::Srcset).boxed(),
                ImgAttributesName::Style => String::new_parser().map_output(Self::Style).boxed(),
                ImgAttributesName::Tabindex => {
                    String::new_parser().map_output(Self::Tabindex).boxed()
                }
                ImgAttributesName::Title => String::new_parser().map_output(Self::Title).boxed(),
                ImgAttributesName::Translate => crate::YValues::new_parser()
                    .map_output(Self::Translate)
                    .boxed(),
                ImgAttributesName::Usemap => String::new_parser().map_output(Self::Usemap).boxed(),
                ImgAttributesName::Width => String::new_parser().map_output(Self::Width).boxed(),
            })
            .map_output(|(_, attribute)| attribute)
    }
}
#[derive(Debug, Clone)]
pub struct Img {
    attributes: Vec<ImgAttributes>,
}
impl kalosm_sample::Parse for Img {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ImgAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Img { attributes })
    }
}
