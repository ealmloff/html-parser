#![allow(clippy::enum_variant_names)]
#![allow(clippy::module_inception)]
#![allow(unused_imports)]
#![allow(dead_code)]
use kalosm_sample::*;
#[derive(Debug, Clone, Copy, Parse)]
pub enum BValues {
    #[parse(rename = "false")]
    False,
    #[parse(rename = "true")]
    True,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum UValues {
    #[parse(rename = "false")]
    False,
    #[parse(rename = "true")]
    True,
    #[parse(rename = "undefined")]
    Undefined,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum OValues {
    #[parse(rename = "off")]
    Off,
    #[parse(rename = "on")]
    On,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum YValues {
    #[parse(rename = "no")]
    No,
    #[parse(rename = "yes")]
    Yes,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum WValues {
    #[parse(rename = "hard")]
    Hard,
    #[parse(rename = "soft")]
    Soft,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum DValues {
    #[parse(rename = "auto")]
    Auto,
    #[parse(rename = "ltr")]
    Ltr,
    #[parse(rename = "rtl")]
    Rtl,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum MValues {
    #[parse(rename = "dialog")]
    Dialog,
    #[parse(rename = "get")]
    Get,
    #[parse(rename = "post")]
    Post,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum FmValues {
    #[parse(rename = "get")]
    Get,
    #[parse(rename = "post")]
    Post,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum SValues {
    #[parse(rename = "col")]
    Col,
    #[parse(rename = "colgroup")]
    Colgroup,
    #[parse(rename = "row")]
    Row,
    #[parse(rename = "rowgroup")]
    Rowgroup,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum TValues {
    #[parse(rename = "button")]
    Button,
    #[parse(rename = "checkbox")]
    Checkbox,
    #[parse(rename = "color")]
    Color,
    #[parse(rename = "date")]
    Date,
    #[parse(rename = "datetime")]
    Datetime,
    #[parse(rename = "datetime-local")]
    DatetimeLocal,
    #[parse(rename = "email")]
    Email,
    #[parse(rename = "file")]
    File,
    #[parse(rename = "hidden")]
    Hidden,
    #[parse(rename = "image")]
    Image,
    #[parse(rename = "month")]
    Month,
    #[parse(rename = "number")]
    Number,
    #[parse(rename = "password")]
    Password,
    #[parse(rename = "radio")]
    Radio,
    #[parse(rename = "range")]
    Range,
    #[parse(rename = "reset")]
    Reset,
    #[parse(rename = "search")]
    Search,
    #[parse(rename = "submit")]
    Submit,
    #[parse(rename = "tel")]
    Tel,
    #[parse(rename = "text")]
    Text,
    #[parse(rename = "time")]
    Time,
    #[parse(rename = "url")]
    Url,
    #[parse(rename = "week")]
    Week,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum ImValues {
    #[parse(rename = "email")]
    Email,
    #[parse(rename = "full-width-latin")]
    FullWidthLatin,
    #[parse(rename = "kana")]
    Kana,
    #[parse(rename = "kana-name")]
    KanaName,
    #[parse(rename = "katakana")]
    Katakana,
    #[parse(rename = "latin")]
    Latin,
    #[parse(rename = "latin-name")]
    LatinName,
    #[parse(rename = "latin-prose")]
    LatinProse,
    #[parse(rename = "numeric")]
    Numeric,
    #[parse(rename = "tel")]
    Tel,
    #[parse(rename = "url")]
    Url,
    #[parse(rename = "verbatim")]
    Verbatim,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum BtValues {
    #[parse(rename = "button")]
    Button,
    #[parse(rename = "menu")]
    Menu,
    #[parse(rename = "reset")]
    Reset,
    #[parse(rename = "submit")]
    Submit,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum LtValues {
    #[parse(rename = "1")]
    Value1,
    #[parse(rename = "A")]
    A,
    #[parse(rename = "I")]
    I,
    #[parse(rename = "a")]
    A2,
    #[parse(rename = "i")]
    I2,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum MtValues {
    #[parse(rename = "context")]
    Context,
    #[parse(rename = "toolbar")]
    Toolbar,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum MitValues {
    #[parse(rename = "checkbox")]
    Checkbox,
    #[parse(rename = "command")]
    Command,
    #[parse(rename = "radio")]
    Radio,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum EtValues {
    #[parse(rename = "application/x-www-form-urlencoded")]
    ApplicationxWwwFormUrlencoded,
    #[parse(rename = "multipart/form-data")]
    MultipartformData,
    #[parse(rename = "text/plain")]
    Textplain,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum TkValues {
    #[parse(rename = "captions")]
    Captions,
    #[parse(rename = "chapters")]
    Chapters,
    #[parse(rename = "descriptions")]
    Descriptions,
    #[parse(rename = "metadata")]
    Metadata,
    #[parse(rename = "subtitles")]
    Subtitles,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum PlValues {
    #[parse(rename = "auto")]
    Auto,
    #[parse(rename = "metadata")]
    Metadata,
    #[parse(rename = "none")]
    None,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum ShValues {
    #[parse(rename = "circle")]
    Circle,
    #[parse(rename = "default")]
    Default,
    #[parse(rename = "poly")]
    Poly,
    #[parse(rename = "rect")]
    Rect,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum XoValues {
    #[parse(rename = "anonymous")]
    Anonymous,
    #[parse(rename = "use-credentials")]
    UseCredentials,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum TargetValues {
    #[parse(rename = "_blank")]
    Blank,
    #[parse(rename = "_parent")]
    Parent,
    #[parse(rename = "_self")]
    ValueSelf,
    #[parse(rename = "_top")]
    Top,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum SbValues {
    #[parse(rename = "allow-forms")]
    AllowForms,
    #[parse(rename = "allow-modals")]
    AllowModals,
    #[parse(rename = "allow-pointer-lock")]
    AllowPointerLock,
    #[parse(rename = "allow-popups")]
    AllowPopups,
    #[parse(rename = "allow-popups-to-escape-sandbox")]
    AllowPopupsToEscapeSandbox,
    #[parse(rename = "allow-same-origin")]
    AllowSameOrigin,
    #[parse(rename = "allow-scripts")]
    AllowScripts,
    #[parse(rename = "allow-top-navigation")]
    AllowTopNavigation,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum TristateValues {
    #[parse(rename = "false")]
    False,
    #[parse(rename = "mixed")]
    Mixed,
    #[parse(rename = "true")]
    True,
    #[parse(rename = "undefined")]
    Undefined,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum InputautocompleteValues {
    #[parse(rename = "additional-name")]
    AdditionalName,
    #[parse(rename = "address-level1")]
    AddressLevel1,
    #[parse(rename = "address-level2")]
    AddressLevel2,
    #[parse(rename = "address-level3")]
    AddressLevel3,
    #[parse(rename = "address-level4")]
    AddressLevel4,
    #[parse(rename = "address-line1")]
    AddressLine1,
    #[parse(rename = "address-line2")]
    AddressLine2,
    #[parse(rename = "address-line3")]
    AddressLine3,
    #[parse(rename = "bday")]
    Bday,
    #[parse(rename = "bday-day")]
    BdayDay,
    #[parse(rename = "bday-month")]
    BdayMonth,
    #[parse(rename = "bday-year")]
    BdayYear,
    #[parse(rename = "billing")]
    Billing,
    #[parse(rename = "cc-additional-name")]
    CcAdditionalName,
    #[parse(rename = "cc-csc")]
    CcCsc,
    #[parse(rename = "cc-exp")]
    CcExp,
    #[parse(rename = "cc-exp-month")]
    CcExpMonth,
    #[parse(rename = "cc-exp-year")]
    CcExpYear,
    #[parse(rename = "cc-family-name")]
    CcFamilyName,
    #[parse(rename = "cc-given-name")]
    CcGivenName,
    #[parse(rename = "cc-name")]
    CcName,
    #[parse(rename = "cc-number")]
    CcNumber,
    #[parse(rename = "cc-type")]
    CcType,
    #[parse(rename = "country")]
    Country,
    #[parse(rename = "country-name")]
    CountryName,
    #[parse(rename = "current-password")]
    CurrentPassword,
    #[parse(rename = "email")]
    Email,
    #[parse(rename = "family-name")]
    FamilyName,
    #[parse(rename = "fax")]
    Fax,
    #[parse(rename = "given-name")]
    GivenName,
    #[parse(rename = "home")]
    Home,
    #[parse(rename = "honorific-prefix")]
    HonorificPrefix,
    #[parse(rename = "honorific-suffix")]
    HonorificSuffix,
    #[parse(rename = "impp")]
    Impp,
    #[parse(rename = "language")]
    Language,
    #[parse(rename = "mobile")]
    Mobile,
    #[parse(rename = "name")]
    Name,
    #[parse(rename = "new-password")]
    NewPassword,
    #[parse(rename = "nickname")]
    Nickname,
    #[parse(rename = "off")]
    Off,
    #[parse(rename = "on")]
    On,
    #[parse(rename = "organization")]
    Organization,
    #[parse(rename = "organization-title")]
    OrganizationTitle,
    #[parse(rename = "pager")]
    Pager,
    #[parse(rename = "photo")]
    Photo,
    #[parse(rename = "postal-code")]
    PostalCode,
    #[parse(rename = "sex")]
    Sex,
    #[parse(rename = "shipping")]
    Shipping,
    #[parse(rename = "street-address")]
    StreetAddress,
    #[parse(rename = "tel")]
    Tel,
    #[parse(rename = "tel-area-code")]
    TelAreaCode,
    #[parse(rename = "tel-country-code")]
    TelCountryCode,
    #[parse(rename = "tel-extension")]
    TelExtension,
    #[parse(rename = "tel-local")]
    TelLocal,
    #[parse(rename = "tel-local-prefix")]
    TelLocalPrefix,
    #[parse(rename = "tel-local-suffix")]
    TelLocalSuffix,
    #[parse(rename = "tel-national")]
    TelNational,
    #[parse(rename = "transaction-amount")]
    TransactionAmount,
    #[parse(rename = "transaction-currency")]
    TransactionCurrency,
    #[parse(rename = "url")]
    Url,
    #[parse(rename = "username")]
    Username,
    #[parse(rename = "work")]
    Work,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum AutocompleteValues {
    #[parse(rename = "both")]
    Both,
    #[parse(rename = "inline")]
    Inline,
    #[parse(rename = "list")]
    List,
    #[parse(rename = "none")]
    None,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum CurrentValues {
    #[parse(rename = "date")]
    Date,
    #[parse(rename = "false")]
    False,
    #[parse(rename = "location")]
    Location,
    #[parse(rename = "page")]
    Page,
    #[parse(rename = "step")]
    Step,
    #[parse(rename = "time")]
    Time,
    #[parse(rename = "true")]
    True,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum DropeffectValues {
    #[parse(rename = "copy")]
    Copy,
    #[parse(rename = "execute")]
    Execute,
    #[parse(rename = "link")]
    Link,
    #[parse(rename = "move")]
    Move,
    #[parse(rename = "none")]
    None,
    #[parse(rename = "popup")]
    Popup,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum InvalidValues {
    #[parse(rename = "false")]
    False,
    #[parse(rename = "grammar")]
    Grammar,
    #[parse(rename = "spelling")]
    Spelling,
    #[parse(rename = "true")]
    True,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum LiveValues {
    #[parse(rename = "assertive")]
    Assertive,
    #[parse(rename = "off")]
    Off,
    #[parse(rename = "polite")]
    Polite,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum OrientationValues {
    #[parse(rename = "horizontal")]
    Horizontal,
    #[parse(rename = "undefined")]
    Undefined,
    #[parse(rename = "vertical")]
    Vertical,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum RelevantValues {
    #[parse(rename = "additions")]
    Additions,
    #[parse(rename = "additions text")]
    AdditionsText,
    #[parse(rename = "all")]
    All,
    #[parse(rename = "removals")]
    Removals,
    #[parse(rename = "text")]
    Text,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum SortValues {
    #[parse(rename = "ascending")]
    Ascending,
    #[parse(rename = "descending")]
    Descending,
    #[parse(rename = "none")]
    None,
    #[parse(rename = "other")]
    Other,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum RolesValues {
    #[parse(rename = "alert")]
    Alert,
    #[parse(rename = "alertdialog")]
    Alertdialog,
    #[parse(rename = "application")]
    Application,
    #[parse(rename = "article")]
    Article,
    #[parse(rename = "banner")]
    Banner,
    #[parse(rename = "button")]
    Button,
    #[parse(rename = "cell")]
    Cell,
    #[parse(rename = "checkbox")]
    Checkbox,
    #[parse(rename = "columnheader")]
    Columnheader,
    #[parse(rename = "combobox")]
    Combobox,
    #[parse(rename = "complementary")]
    Complementary,
    #[parse(rename = "contentinfo")]
    Contentinfo,
    #[parse(rename = "definition")]
    Definition,
    #[parse(rename = "dialog")]
    Dialog,
    #[parse(rename = "directory")]
    Directory,
    #[parse(rename = "doc-abstract")]
    DocAbstract,
    #[parse(rename = "doc-acknowledgments")]
    DocAcknowledgments,
    #[parse(rename = "doc-afterword")]
    DocAfterword,
    #[parse(rename = "doc-appendix")]
    DocAppendix,
    #[parse(rename = "doc-backlink")]
    DocBacklink,
    #[parse(rename = "doc-biblioentry")]
    DocBiblioentry,
    #[parse(rename = "doc-bibliography")]
    DocBibliography,
    #[parse(rename = "doc-biblioref")]
    DocBiblioref,
    #[parse(rename = "doc-chapter")]
    DocChapter,
    #[parse(rename = "doc-colophon")]
    DocColophon,
    #[parse(rename = "doc-conclusion")]
    DocConclusion,
    #[parse(rename = "doc-cover")]
    DocCover,
    #[parse(rename = "doc-credit")]
    DocCredit,
    #[parse(rename = "doc-credits")]
    DocCredits,
    #[parse(rename = "doc-dedication")]
    DocDedication,
    #[parse(rename = "doc-endnote")]
    DocEndnote,
    #[parse(rename = "doc-endnotes")]
    DocEndnotes,
    #[parse(rename = "doc-epigraph")]
    DocEpigraph,
    #[parse(rename = "doc-epilogue")]
    DocEpilogue,
    #[parse(rename = "doc-errata")]
    DocErrata,
    #[parse(rename = "doc-example")]
    DocExample,
    #[parse(rename = "doc-footnote")]
    DocFootnote,
    #[parse(rename = "doc-foreword")]
    DocForeword,
    #[parse(rename = "doc-glossary")]
    DocGlossary,
    #[parse(rename = "doc-glossref")]
    DocGlossref,
    #[parse(rename = "doc-index")]
    DocIndex,
    #[parse(rename = "doc-introduction")]
    DocIntroduction,
    #[parse(rename = "doc-noteref")]
    DocNoteref,
    #[parse(rename = "doc-notice")]
    DocNotice,
    #[parse(rename = "doc-pagebreak")]
    DocPagebreak,
    #[parse(rename = "doc-pagelist")]
    DocPagelist,
    #[parse(rename = "doc-part")]
    DocPart,
    #[parse(rename = "doc-preface")]
    DocPreface,
    #[parse(rename = "doc-prologue")]
    DocPrologue,
    #[parse(rename = "doc-pullquote")]
    DocPullquote,
    #[parse(rename = "doc-qna")]
    DocQna,
    #[parse(rename = "doc-subtitle")]
    DocSubtitle,
    #[parse(rename = "doc-tip")]
    DocTip,
    #[parse(rename = "doc-toc")]
    DocToc,
    #[parse(rename = "document")]
    Document,
    #[parse(rename = "feed")]
    Feed,
    #[parse(rename = "figure")]
    Figure,
    #[parse(rename = "form")]
    Form,
    #[parse(rename = "grid")]
    Grid,
    #[parse(rename = "gridcell")]
    Gridcell,
    #[parse(rename = "group")]
    Group,
    #[parse(rename = "heading")]
    Heading,
    #[parse(rename = "img")]
    Img,
    #[parse(rename = "link")]
    Link,
    #[parse(rename = "list")]
    List,
    #[parse(rename = "listbox")]
    Listbox,
    #[parse(rename = "listitem")]
    Listitem,
    #[parse(rename = "log")]
    Log,
    #[parse(rename = "main")]
    Main,
    #[parse(rename = "marquee")]
    Marquee,
    #[parse(rename = "math")]
    Math,
    #[parse(rename = "menu")]
    Menu,
    #[parse(rename = "menubar")]
    Menubar,
    #[parse(rename = "menuitem")]
    Menuitem,
    #[parse(rename = "menuitemcheckbox")]
    Menuitemcheckbox,
    #[parse(rename = "menuitemradio")]
    Menuitemradio,
    #[parse(rename = "navigation")]
    Navigation,
    #[parse(rename = "none")]
    None,
    #[parse(rename = "note")]
    Note,
    #[parse(rename = "option")]
    Option,
    #[parse(rename = "presentation")]
    Presentation,
    #[parse(rename = "progressbar")]
    Progressbar,
    #[parse(rename = "radio")]
    Radio,
    #[parse(rename = "radiogroup")]
    Radiogroup,
    #[parse(rename = "region")]
    Region,
    #[parse(rename = "row")]
    Row,
    #[parse(rename = "rowgroup")]
    Rowgroup,
    #[parse(rename = "rowheader")]
    Rowheader,
    #[parse(rename = "scrollbar")]
    Scrollbar,
    #[parse(rename = "search")]
    Search,
    #[parse(rename = "searchbox")]
    Searchbox,
    #[parse(rename = "separator")]
    Separator,
    #[parse(rename = "slider")]
    Slider,
    #[parse(rename = "spinbutton")]
    Spinbutton,
    #[parse(rename = "status")]
    Status,
    #[parse(rename = "switch")]
    Switch,
    #[parse(rename = "tab")]
    Tab,
    #[parse(rename = "table")]
    Table,
    #[parse(rename = "tablist")]
    Tablist,
    #[parse(rename = "tabpanel")]
    Tabpanel,
    #[parse(rename = "term")]
    Term,
    #[parse(rename = "text")]
    Text,
    #[parse(rename = "textbox")]
    Textbox,
    #[parse(rename = "timer")]
    Timer,
    #[parse(rename = "toolbar")]
    Toolbar,
    #[parse(rename = "tooltip")]
    Tooltip,
    #[parse(rename = "tree")]
    Tree,
    #[parse(rename = "treegrid")]
    Treegrid,
    #[parse(rename = "treeitem")]
    Treeitem,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum MetanamesValues {
    #[parse(rename = "application-name")]
    ApplicationName,
    #[parse(rename = "author")]
    Author,
    #[parse(rename = "description")]
    Description,
    #[parse(rename = "format-detection")]
    FormatDetection,
    #[parse(rename = "generator")]
    Generator,
    #[parse(rename = "keywords")]
    Keywords,
    #[parse(rename = "publisher")]
    Publisher,
    #[parse(rename = "referrer")]
    Referrer,
    #[parse(rename = "robots")]
    Robots,
    #[parse(rename = "theme-color")]
    ThemeColor,
    #[parse(rename = "viewport")]
    Viewport,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum HaspopupValues {
    #[parse(rename = "dialog")]
    Dialog,
    #[parse(rename = "false")]
    False,
    #[parse(rename = "grid")]
    Grid,
    #[parse(rename = "listbox")]
    Listbox,
    #[parse(rename = "menu")]
    Menu,
    #[parse(rename = "tree")]
    Tree,
    #[parse(rename = "true")]
    True,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum DecodingValues {
    #[parse(rename = "async")]
    Async,
    #[parse(rename = "auto")]
    Auto,
    #[parse(rename = "sync")]
    Sync,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum LoadingValues {
    #[parse(rename = "eager")]
    Eager,
    #[parse(rename = "lazy")]
    Lazy,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum ReferrerpolicyValues {
    #[parse(rename = "no-referrer")]
    NoReferrer,
    #[parse(rename = "no-referrer-when-downgrade")]
    NoReferrerWhenDowngrade,
    #[parse(rename = "origin")]
    Origin,
    #[parse(rename = "origin-when-cross-origin")]
    OriginWhenCrossOrigin,
    #[parse(rename = "same-origin")]
    SameOrigin,
    #[parse(rename = "strict-origin")]
    StrictOrigin,
    #[parse(rename = "strict-origin-when-cross-origin")]
    StrictOriginWhenCrossOrigin,
    #[parse(rename = "unsafe-url")]
    UnsafeUrl,
}
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum GlobalAttributeName {
    #[parse(rename = " accesskey=")]
    Accesskey,
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
    #[parse(rename = " onmouseenter=")]
    Onmouseenter,
    #[parse(rename = " onmouseleave=")]
    Onmouseleave,
    #[parse(rename = " onpause=")]
    Onpause,
    #[parse(rename = " onplay=")]
    Onplay,
    #[parse(rename = " onplaying=")]
    Onplaying,
    #[parse(rename = " onprogress=")]
    Onprogress,
    #[parse(rename = " onratechange=")]
    Onratechange,
    #[parse(rename = " onreset=")]
    Onreset,
    #[parse(rename = " onresize=")]
    Onresize,
    #[parse(rename = " onreadystatechange=")]
    Onreadystatechange,
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
    #[parse(rename = " aria-details=")]
    AriaDetails,
    #[parse(rename = " aria-keyshortcuts=")]
    AriaKeyshortcuts,
}
#[derive(Debug, Clone)]
pub enum GlobalAttribute {
    Accesskey(String),
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
    Part(String),
    Role(crate::RolesValues),
    Slot(String),
    Spellcheck(crate::BValues),
    Style(String),
    Tabindex(String),
    Title(String),
    Translate(crate::YValues),
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
    Onmousemove(String),
    Onmouseout(String),
    Onmouseover(String),
    Onmouseup(String),
    Onmousewheel(String),
    Onmouseenter(String),
    Onmouseleave(String),
    Onpause(String),
    Onplay(String),
    Onplaying(String),
    Onprogress(String),
    Onratechange(String),
    Onreset(String),
    Onresize(String),
    Onreadystatechange(String),
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
    AriaDisabled(crate::BValues),
    AriaDropeffect(crate::DropeffectValues),
    AriaErrormessage(String),
    AriaExpanded(crate::UValues),
    AriaFlowto(String),
    AriaGrabbed(crate::UValues),
    AriaHaspopup(crate::HaspopupValues),
    AriaHidden(crate::BValues),
    AriaInvalid(crate::InvalidValues),
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
    AriaDetails(String),
    AriaKeyshortcuts(String),
}

impl kalosm_sample::Parse for GlobalAttribute {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        GlobalAttributeName::new_parser()
            .then_lazy(|name| match name {
                GlobalAttributeName::Accesskey => {
                    String::new_parser().map_output(Self::Accesskey).boxed()
                }
                GlobalAttributeName::Autocapitalize => String::new_parser()
                    .map_output(Self::Autocapitalize)
                    .boxed(),
                GlobalAttributeName::Class => String::new_parser().map_output(Self::Class).boxed(),
                GlobalAttributeName::Contenteditable => String::new_parser()
                    .map_output(Self::Contenteditable)
                    .boxed(),
                GlobalAttributeName::Contextmenu => {
                    String::new_parser().map_output(Self::Contextmenu).boxed()
                }
                GlobalAttributeName::Dir => {
                    crate::DValues::new_parser().map_output(Self::Dir).boxed()
                }
                GlobalAttributeName::Draggable => crate::BValues::new_parser()
                    .map_output(Self::Draggable)
                    .boxed(),
                GlobalAttributeName::Dropzone => {
                    String::new_parser().map_output(Self::Dropzone).boxed()
                }
                GlobalAttributeName::Exportparts => {
                    String::new_parser().map_output(Self::Exportparts).boxed()
                }
                GlobalAttributeName::Hidden => {
                    String::new_parser().map_output(Self::Hidden).boxed()
                }
                GlobalAttributeName::Id => String::new_parser().map_output(Self::Id).boxed(),
                GlobalAttributeName::Inputmode => {
                    String::new_parser().map_output(Self::Inputmode).boxed()
                }
                GlobalAttributeName::Is => String::new_parser().map_output(Self::Is).boxed(),
                GlobalAttributeName::Itemid => {
                    String::new_parser().map_output(Self::Itemid).boxed()
                }
                GlobalAttributeName::Itemprop => {
                    String::new_parser().map_output(Self::Itemprop).boxed()
                }
                GlobalAttributeName::Itemref => {
                    String::new_parser().map_output(Self::Itemref).boxed()
                }
                GlobalAttributeName::Itemscope => {
                    String::new_parser().map_output(Self::Itemscope).boxed()
                }
                GlobalAttributeName::Itemtype => {
                    String::new_parser().map_output(Self::Itemtype).boxed()
                }
                GlobalAttributeName::Lang => String::new_parser().map_output(Self::Lang).boxed(),
                GlobalAttributeName::Part => String::new_parser().map_output(Self::Part).boxed(),
                GlobalAttributeName::Role => crate::RolesValues::new_parser()
                    .map_output(Self::Role)
                    .boxed(),
                GlobalAttributeName::Slot => String::new_parser().map_output(Self::Slot).boxed(),
                GlobalAttributeName::Spellcheck => crate::BValues::new_parser()
                    .map_output(Self::Spellcheck)
                    .boxed(),
                GlobalAttributeName::Style => String::new_parser().map_output(Self::Style).boxed(),
                GlobalAttributeName::Tabindex => {
                    String::new_parser().map_output(Self::Tabindex).boxed()
                }
                GlobalAttributeName::Title => String::new_parser().map_output(Self::Title).boxed(),
                GlobalAttributeName::Translate => crate::YValues::new_parser()
                    .map_output(Self::Translate)
                    .boxed(),
                GlobalAttributeName::Onabort => {
                    String::new_parser().map_output(Self::Onabort).boxed()
                }
                GlobalAttributeName::Onblur => {
                    String::new_parser().map_output(Self::Onblur).boxed()
                }
                GlobalAttributeName::Oncanplay => {
                    String::new_parser().map_output(Self::Oncanplay).boxed()
                }
                GlobalAttributeName::Oncanplaythrough => String::new_parser()
                    .map_output(Self::Oncanplaythrough)
                    .boxed(),
                GlobalAttributeName::Onchange => {
                    String::new_parser().map_output(Self::Onchange).boxed()
                }
                GlobalAttributeName::Onclick => {
                    String::new_parser().map_output(Self::Onclick).boxed()
                }
                GlobalAttributeName::Oncontextmenu => {
                    String::new_parser().map_output(Self::Oncontextmenu).boxed()
                }
                GlobalAttributeName::Ondblclick => {
                    String::new_parser().map_output(Self::Ondblclick).boxed()
                }
                GlobalAttributeName::Ondrag => {
                    String::new_parser().map_output(Self::Ondrag).boxed()
                }
                GlobalAttributeName::Ondragend => {
                    String::new_parser().map_output(Self::Ondragend).boxed()
                }
                GlobalAttributeName::Ondragenter => {
                    String::new_parser().map_output(Self::Ondragenter).boxed()
                }
                GlobalAttributeName::Ondragleave => {
                    String::new_parser().map_output(Self::Ondragleave).boxed()
                }
                GlobalAttributeName::Ondragover => {
                    String::new_parser().map_output(Self::Ondragover).boxed()
                }
                GlobalAttributeName::Ondragstart => {
                    String::new_parser().map_output(Self::Ondragstart).boxed()
                }
                GlobalAttributeName::Ondrop => {
                    String::new_parser().map_output(Self::Ondrop).boxed()
                }
                GlobalAttributeName::Ondurationchange => String::new_parser()
                    .map_output(Self::Ondurationchange)
                    .boxed(),
                GlobalAttributeName::Onemptied => {
                    String::new_parser().map_output(Self::Onemptied).boxed()
                }
                GlobalAttributeName::Onended => {
                    String::new_parser().map_output(Self::Onended).boxed()
                }
                GlobalAttributeName::Onerror => {
                    String::new_parser().map_output(Self::Onerror).boxed()
                }
                GlobalAttributeName::Onfocus => {
                    String::new_parser().map_output(Self::Onfocus).boxed()
                }
                GlobalAttributeName::Onformchange => {
                    String::new_parser().map_output(Self::Onformchange).boxed()
                }
                GlobalAttributeName::Onforminput => {
                    String::new_parser().map_output(Self::Onforminput).boxed()
                }
                GlobalAttributeName::Oninput => {
                    String::new_parser().map_output(Self::Oninput).boxed()
                }
                GlobalAttributeName::Oninvalid => {
                    String::new_parser().map_output(Self::Oninvalid).boxed()
                }
                GlobalAttributeName::Onkeydown => {
                    String::new_parser().map_output(Self::Onkeydown).boxed()
                }
                GlobalAttributeName::Onkeypress => {
                    String::new_parser().map_output(Self::Onkeypress).boxed()
                }
                GlobalAttributeName::Onkeyup => {
                    String::new_parser().map_output(Self::Onkeyup).boxed()
                }
                GlobalAttributeName::Onload => {
                    String::new_parser().map_output(Self::Onload).boxed()
                }
                GlobalAttributeName::Onloadeddata => {
                    String::new_parser().map_output(Self::Onloadeddata).boxed()
                }
                GlobalAttributeName::Onloadedmetadata => String::new_parser()
                    .map_output(Self::Onloadedmetadata)
                    .boxed(),
                GlobalAttributeName::Onloadstart => {
                    String::new_parser().map_output(Self::Onloadstart).boxed()
                }
                GlobalAttributeName::Onmousedown => {
                    String::new_parser().map_output(Self::Onmousedown).boxed()
                }
                GlobalAttributeName::Onmousemove => {
                    String::new_parser().map_output(Self::Onmousemove).boxed()
                }
                GlobalAttributeName::Onmouseout => {
                    String::new_parser().map_output(Self::Onmouseout).boxed()
                }
                GlobalAttributeName::Onmouseover => {
                    String::new_parser().map_output(Self::Onmouseover).boxed()
                }
                GlobalAttributeName::Onmouseup => {
                    String::new_parser().map_output(Self::Onmouseup).boxed()
                }
                GlobalAttributeName::Onmousewheel => {
                    String::new_parser().map_output(Self::Onmousewheel).boxed()
                }
                GlobalAttributeName::Onmouseenter => {
                    String::new_parser().map_output(Self::Onmouseenter).boxed()
                }
                GlobalAttributeName::Onmouseleave => {
                    String::new_parser().map_output(Self::Onmouseleave).boxed()
                }
                GlobalAttributeName::Onpause => {
                    String::new_parser().map_output(Self::Onpause).boxed()
                }
                GlobalAttributeName::Onplay => {
                    String::new_parser().map_output(Self::Onplay).boxed()
                }
                GlobalAttributeName::Onplaying => {
                    String::new_parser().map_output(Self::Onplaying).boxed()
                }
                GlobalAttributeName::Onprogress => {
                    String::new_parser().map_output(Self::Onprogress).boxed()
                }
                GlobalAttributeName::Onratechange => {
                    String::new_parser().map_output(Self::Onratechange).boxed()
                }
                GlobalAttributeName::Onreset => {
                    String::new_parser().map_output(Self::Onreset).boxed()
                }
                GlobalAttributeName::Onresize => {
                    String::new_parser().map_output(Self::Onresize).boxed()
                }
                GlobalAttributeName::Onreadystatechange => String::new_parser()
                    .map_output(Self::Onreadystatechange)
                    .boxed(),
                GlobalAttributeName::Onscroll => {
                    String::new_parser().map_output(Self::Onscroll).boxed()
                }
                GlobalAttributeName::Onseeked => {
                    String::new_parser().map_output(Self::Onseeked).boxed()
                }
                GlobalAttributeName::Onseeking => {
                    String::new_parser().map_output(Self::Onseeking).boxed()
                }
                GlobalAttributeName::Onselect => {
                    String::new_parser().map_output(Self::Onselect).boxed()
                }
                GlobalAttributeName::Onshow => {
                    String::new_parser().map_output(Self::Onshow).boxed()
                }
                GlobalAttributeName::Onstalled => {
                    String::new_parser().map_output(Self::Onstalled).boxed()
                }
                GlobalAttributeName::Onsubmit => {
                    String::new_parser().map_output(Self::Onsubmit).boxed()
                }
                GlobalAttributeName::Onsuspend => {
                    String::new_parser().map_output(Self::Onsuspend).boxed()
                }
                GlobalAttributeName::Ontimeupdate => {
                    String::new_parser().map_output(Self::Ontimeupdate).boxed()
                }
                GlobalAttributeName::Onvolumechange => String::new_parser()
                    .map_output(Self::Onvolumechange)
                    .boxed(),
                GlobalAttributeName::Onwaiting => {
                    String::new_parser().map_output(Self::Onwaiting).boxed()
                }
                GlobalAttributeName::Onpointercancel => String::new_parser()
                    .map_output(Self::Onpointercancel)
                    .boxed(),
                GlobalAttributeName::Onpointerdown => {
                    String::new_parser().map_output(Self::Onpointerdown).boxed()
                }
                GlobalAttributeName::Onpointerenter => String::new_parser()
                    .map_output(Self::Onpointerenter)
                    .boxed(),
                GlobalAttributeName::Onpointerleave => String::new_parser()
                    .map_output(Self::Onpointerleave)
                    .boxed(),
                GlobalAttributeName::Onpointerlockchange => String::new_parser()
                    .map_output(Self::Onpointerlockchange)
                    .boxed(),
                GlobalAttributeName::Onpointerlockerror => String::new_parser()
                    .map_output(Self::Onpointerlockerror)
                    .boxed(),
                GlobalAttributeName::Onpointermove => {
                    String::new_parser().map_output(Self::Onpointermove).boxed()
                }
                GlobalAttributeName::Onpointerout => {
                    String::new_parser().map_output(Self::Onpointerout).boxed()
                }
                GlobalAttributeName::Onpointerover => {
                    String::new_parser().map_output(Self::Onpointerover).boxed()
                }
                GlobalAttributeName::Onpointerup => {
                    String::new_parser().map_output(Self::Onpointerup).boxed()
                }
                GlobalAttributeName::AriaActivedescendant => String::new_parser()
                    .map_output(Self::AriaActivedescendant)
                    .boxed(),
                GlobalAttributeName::AriaAtomic => crate::BValues::new_parser()
                    .map_output(Self::AriaAtomic)
                    .boxed(),
                GlobalAttributeName::AriaAutocomplete => crate::AutocompleteValues::new_parser()
                    .map_output(Self::AriaAutocomplete)
                    .boxed(),
                GlobalAttributeName::AriaBusy => crate::BValues::new_parser()
                    .map_output(Self::AriaBusy)
                    .boxed(),
                GlobalAttributeName::AriaChecked => crate::TristateValues::new_parser()
                    .map_output(Self::AriaChecked)
                    .boxed(),
                GlobalAttributeName::AriaColcount => {
                    String::new_parser().map_output(Self::AriaColcount).boxed()
                }
                GlobalAttributeName::AriaColindex => {
                    String::new_parser().map_output(Self::AriaColindex).boxed()
                }
                GlobalAttributeName::AriaColspan => {
                    String::new_parser().map_output(Self::AriaColspan).boxed()
                }
                GlobalAttributeName::AriaControls => {
                    String::new_parser().map_output(Self::AriaControls).boxed()
                }
                GlobalAttributeName::AriaCurrent => crate::CurrentValues::new_parser()
                    .map_output(Self::AriaCurrent)
                    .boxed(),
                GlobalAttributeName::AriaDescribedby => String::new_parser()
                    .map_output(Self::AriaDescribedby)
                    .boxed(),
                GlobalAttributeName::AriaDisabled => crate::BValues::new_parser()
                    .map_output(Self::AriaDisabled)
                    .boxed(),
                GlobalAttributeName::AriaDropeffect => crate::DropeffectValues::new_parser()
                    .map_output(Self::AriaDropeffect)
                    .boxed(),
                GlobalAttributeName::AriaErrormessage => String::new_parser()
                    .map_output(Self::AriaErrormessage)
                    .boxed(),
                GlobalAttributeName::AriaExpanded => crate::UValues::new_parser()
                    .map_output(Self::AriaExpanded)
                    .boxed(),
                GlobalAttributeName::AriaFlowto => {
                    String::new_parser().map_output(Self::AriaFlowto).boxed()
                }
                GlobalAttributeName::AriaGrabbed => crate::UValues::new_parser()
                    .map_output(Self::AriaGrabbed)
                    .boxed(),
                GlobalAttributeName::AriaHaspopup => crate::HaspopupValues::new_parser()
                    .map_output(Self::AriaHaspopup)
                    .boxed(),
                GlobalAttributeName::AriaHidden => crate::BValues::new_parser()
                    .map_output(Self::AriaHidden)
                    .boxed(),
                GlobalAttributeName::AriaInvalid => crate::InvalidValues::new_parser()
                    .map_output(Self::AriaInvalid)
                    .boxed(),
                GlobalAttributeName::AriaLabel => {
                    String::new_parser().map_output(Self::AriaLabel).boxed()
                }
                GlobalAttributeName::AriaLabelledby => String::new_parser()
                    .map_output(Self::AriaLabelledby)
                    .boxed(),
                GlobalAttributeName::AriaLevel => {
                    String::new_parser().map_output(Self::AriaLevel).boxed()
                }
                GlobalAttributeName::AriaLive => crate::LiveValues::new_parser()
                    .map_output(Self::AriaLive)
                    .boxed(),
                GlobalAttributeName::AriaModal => crate::BValues::new_parser()
                    .map_output(Self::AriaModal)
                    .boxed(),
                GlobalAttributeName::AriaMultiline => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiline)
                    .boxed(),
                GlobalAttributeName::AriaMultiselectable => crate::BValues::new_parser()
                    .map_output(Self::AriaMultiselectable)
                    .boxed(),
                GlobalAttributeName::AriaOrientation => crate::OrientationValues::new_parser()
                    .map_output(Self::AriaOrientation)
                    .boxed(),
                GlobalAttributeName::AriaOwns => {
                    String::new_parser().map_output(Self::AriaOwns).boxed()
                }
                GlobalAttributeName::AriaPlaceholder => String::new_parser()
                    .map_output(Self::AriaPlaceholder)
                    .boxed(),
                GlobalAttributeName::AriaPosinset => {
                    String::new_parser().map_output(Self::AriaPosinset).boxed()
                }
                GlobalAttributeName::AriaPressed => crate::TristateValues::new_parser()
                    .map_output(Self::AriaPressed)
                    .boxed(),
                GlobalAttributeName::AriaReadonly => crate::BValues::new_parser()
                    .map_output(Self::AriaReadonly)
                    .boxed(),
                GlobalAttributeName::AriaRelevant => crate::RelevantValues::new_parser()
                    .map_output(Self::AriaRelevant)
                    .boxed(),
                GlobalAttributeName::AriaRequired => crate::BValues::new_parser()
                    .map_output(Self::AriaRequired)
                    .boxed(),
                GlobalAttributeName::AriaRoledescription => String::new_parser()
                    .map_output(Self::AriaRoledescription)
                    .boxed(),
                GlobalAttributeName::AriaRowcount => {
                    String::new_parser().map_output(Self::AriaRowcount).boxed()
                }
                GlobalAttributeName::AriaRowindex => {
                    String::new_parser().map_output(Self::AriaRowindex).boxed()
                }
                GlobalAttributeName::AriaRowspan => {
                    String::new_parser().map_output(Self::AriaRowspan).boxed()
                }
                GlobalAttributeName::AriaSelected => crate::UValues::new_parser()
                    .map_output(Self::AriaSelected)
                    .boxed(),
                GlobalAttributeName::AriaSetsize => {
                    String::new_parser().map_output(Self::AriaSetsize).boxed()
                }
                GlobalAttributeName::AriaSort => crate::SortValues::new_parser()
                    .map_output(Self::AriaSort)
                    .boxed(),
                GlobalAttributeName::AriaValuemax => {
                    String::new_parser().map_output(Self::AriaValuemax).boxed()
                }
                GlobalAttributeName::AriaValuemin => {
                    String::new_parser().map_output(Self::AriaValuemin).boxed()
                }
                GlobalAttributeName::AriaValuenow => {
                    String::new_parser().map_output(Self::AriaValuenow).boxed()
                }
                GlobalAttributeName::AriaValuetext => {
                    String::new_parser().map_output(Self::AriaValuetext).boxed()
                }
                GlobalAttributeName::AriaDetails => {
                    String::new_parser().map_output(Self::AriaDetails).boxed()
                }
                GlobalAttributeName::AriaKeyshortcuts => String::new_parser()
                    .map_output(Self::AriaKeyshortcuts)
                    .boxed(),
            })
            .map_output(|(_, attribute)| attribute)
    }
}
mod a;
pub use a::*;

mod abbr;
pub use abbr::*;

mod address;
pub use address::*;

mod area;
pub use area::*;

mod article;
pub use article::*;

mod aside;
pub use aside::*;

mod audio;
pub use audio::*;

mod b;
pub use b::*;

mod base;
pub use base::*;

mod bdi;
pub use bdi::*;

mod bdo;
pub use bdo::*;

mod blockquote;
pub use blockquote::*;

mod body;
pub use body::*;

mod br;
pub use br::*;

mod button;
pub use button::*;

mod canvas;
pub use canvas::*;

mod caption;
pub use caption::*;

mod cite;
pub use cite::*;

mod code;
pub use code::*;

mod col;
pub use col::*;

mod colgroup;
pub use colgroup::*;

mod data;
pub use data::*;

mod datalist;
pub use datalist::*;

mod dd;
pub use dd::*;

mod del;
pub use del::*;

mod details;
pub use details::*;

mod dfn;
pub use dfn::*;

mod dialog;
pub use dialog::*;

mod div;
pub use div::*;

mod dl;
pub use dl::*;

mod dt;
pub use dt::*;

mod em;
pub use em::*;

mod embed;
pub use embed::*;

mod fieldset;
pub use fieldset::*;

mod figcaption;
pub use figcaption::*;

mod figure;
pub use figure::*;

mod footer;
pub use footer::*;

mod form;
pub use form::*;

mod h1;
pub use h1::*;

mod h2;
pub use h2::*;

mod h3;
pub use h3::*;

mod h4;
pub use h4::*;

mod h5;
pub use h5::*;

mod h6;
pub use h6::*;

mod head;
pub use head::*;

mod header;
pub use header::*;

mod hgroup;
pub use hgroup::*;

mod hr;
pub use hr::*;

mod html;
pub use html::*;

mod i;
pub use i::*;

mod iframe;
pub use iframe::*;

mod img;
pub use img::*;

mod input;
pub use input::*;

mod ins;
pub use ins::*;

mod kbd;
pub use kbd::*;

mod label;
pub use label::*;

mod legend;
pub use legend::*;

mod li;
pub use li::*;

mod link;
pub use link::*;

mod main;
pub use main::*;

mod map;
pub use map::*;

mod mark;
pub use mark::*;

mod menu;
pub use menu::*;

mod meta;
pub use meta::*;

mod meter;
pub use meter::*;

mod nav;
pub use nav::*;

mod noscript;
pub use noscript::*;

mod object;
pub use object::*;

mod ol;
pub use ol::*;

mod optgroup;
pub use optgroup::*;

mod option;
pub use option::*;

mod output;
pub use output::*;

mod p;
pub use p::*;

mod param;
pub use param::*;

mod picture;
pub use picture::*;

mod pre;
pub use pre::*;

mod progress;
pub use progress::*;

mod q;
pub use q::*;

mod rb;
pub use rb::*;

mod rp;
pub use rp::*;

mod rt;
pub use rt::*;

mod ruby;
pub use ruby::*;

mod s;
pub use s::*;

mod samp;
pub use samp::*;

mod script;
pub use script::*;

mod section;
pub use section::*;

mod select;
pub use select::*;

mod slot;
pub use slot::*;

mod small;
pub use small::*;

mod source;
pub use source::*;

mod span;
pub use span::*;

mod strong;
pub use strong::*;

mod style;
pub use style::*;

mod sub;
pub use sub::*;

mod summary;
pub use summary::*;

mod sup;
pub use sup::*;

mod table;
pub use table::*;

mod tbody;
pub use tbody::*;

mod td;
pub use td::*;

mod template;
pub use template::*;

mod textarea;
pub use textarea::*;

mod tfoot;
pub use tfoot::*;

mod th;
pub use th::*;

mod thead;
pub use thead::*;

mod time;
pub use time::*;

mod title;
pub use title::*;

mod tr;
pub use tr::*;

mod track;
pub use track::*;

mod u;
pub use u::*;

mod ul;
pub use ul::*;

mod var;
pub use var::*;

mod video;
pub use video::*;

mod wbr;
pub use wbr::*;

#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ElementName {
    #[parse(rename = "<a")]
    A,
    #[parse(rename = "<abbr")]
    Abbr,
    #[parse(rename = "<address")]
    Address,
    #[parse(rename = "<area")]
    Area,
    #[parse(rename = "<article")]
    Article,
    #[parse(rename = "<aside")]
    Aside,
    #[parse(rename = "<audio")]
    Audio,
    #[parse(rename = "<b")]
    B,
    #[parse(rename = "<base")]
    Base,
    #[parse(rename = "<bdi")]
    Bdi,
    #[parse(rename = "<bdo")]
    Bdo,
    #[parse(rename = "<blockquote")]
    Blockquote,
    #[parse(rename = "<body")]
    Body,
    #[parse(rename = "<br")]
    Br,
    #[parse(rename = "<button")]
    Button,
    #[parse(rename = "<canvas")]
    Canvas,
    #[parse(rename = "<caption")]
    Caption,
    #[parse(rename = "<cite")]
    Cite,
    #[parse(rename = "<code")]
    Code,
    #[parse(rename = "<col")]
    Col,
    #[parse(rename = "<colgroup")]
    Colgroup,
    #[parse(rename = "<data")]
    Data,
    #[parse(rename = "<datalist")]
    Datalist,
    #[parse(rename = "<dd")]
    Dd,
    #[parse(rename = "<del")]
    Del,
    #[parse(rename = "<details")]
    Details,
    #[parse(rename = "<dfn")]
    Dfn,
    #[parse(rename = "<dialog")]
    Dialog,
    #[parse(rename = "<div")]
    Div,
    #[parse(rename = "<dl")]
    Dl,
    #[parse(rename = "<dt")]
    Dt,
    #[parse(rename = "<em")]
    Em,
    #[parse(rename = "<embed")]
    Embed,
    #[parse(rename = "<fieldset")]
    Fieldset,
    #[parse(rename = "<figcaption")]
    Figcaption,
    #[parse(rename = "<figure")]
    Figure,
    #[parse(rename = "<footer")]
    Footer,
    #[parse(rename = "<form")]
    Form,
    #[parse(rename = "<h1")]
    H1,
    #[parse(rename = "<h2")]
    H2,
    #[parse(rename = "<h3")]
    H3,
    #[parse(rename = "<h4")]
    H4,
    #[parse(rename = "<h5")]
    H5,
    #[parse(rename = "<h6")]
    H6,
    #[parse(rename = "<head")]
    Head,
    #[parse(rename = "<header")]
    Header,
    #[parse(rename = "<hgroup")]
    Hgroup,
    #[parse(rename = "<hr")]
    Hr,
    #[parse(rename = "<html")]
    Html,
    #[parse(rename = "<i")]
    I,
    #[parse(rename = "<iframe")]
    Iframe,
    #[parse(rename = "<img")]
    Img,
    #[parse(rename = "<input")]
    Input,
    #[parse(rename = "<ins")]
    Ins,
    #[parse(rename = "<kbd")]
    Kbd,
    #[parse(rename = "<label")]
    Label,
    #[parse(rename = "<legend")]
    Legend,
    #[parse(rename = "<li")]
    Li,
    #[parse(rename = "<link")]
    Link,
    #[parse(rename = "<main")]
    Main,
    #[parse(rename = "<map")]
    Map,
    #[parse(rename = "<mark")]
    Mark,
    #[parse(rename = "<menu")]
    Menu,
    #[parse(rename = "<meta")]
    Meta,
    #[parse(rename = "<meter")]
    Meter,
    #[parse(rename = "<nav")]
    Nav,
    #[parse(rename = "<noscript")]
    Noscript,
    #[parse(rename = "<object")]
    Object,
    #[parse(rename = "<ol")]
    Ol,
    #[parse(rename = "<optgroup")]
    Optgroup,
    #[parse(rename = "<option")]
    Option,
    #[parse(rename = "<output")]
    Output,
    #[parse(rename = "<p")]
    P,
    #[parse(rename = "<param")]
    Param,
    #[parse(rename = "<picture")]
    Picture,
    #[parse(rename = "<pre")]
    Pre,
    #[parse(rename = "<progress")]
    Progress,
    #[parse(rename = "<q")]
    Q,
    #[parse(rename = "<rb")]
    Rb,
    #[parse(rename = "<rp")]
    Rp,
    #[parse(rename = "<rt")]
    Rt,
    #[parse(rename = "<ruby")]
    Ruby,
    #[parse(rename = "<s")]
    S,
    #[parse(rename = "<samp")]
    Samp,
    #[parse(rename = "<script")]
    Script,
    #[parse(rename = "<section")]
    Section,
    #[parse(rename = "<select")]
    Select,
    #[parse(rename = "<slot")]
    Slot,
    #[parse(rename = "<small")]
    Small,
    #[parse(rename = "<source")]
    Source,
    #[parse(rename = "<span")]
    Span,
    #[parse(rename = "<strong")]
    Strong,
    #[parse(rename = "<style")]
    Style,
    #[parse(rename = "<sub")]
    Sub,
    #[parse(rename = "<summary")]
    Summary,
    #[parse(rename = "<sup")]
    Sup,
    #[parse(rename = "<table")]
    Table,
    #[parse(rename = "<tbody")]
    Tbody,
    #[parse(rename = "<td")]
    Td,
    #[parse(rename = "<template")]
    Template,
    #[parse(rename = "<textarea")]
    Textarea,
    #[parse(rename = "<tfoot")]
    Tfoot,
    #[parse(rename = "<th")]
    Th,
    #[parse(rename = "<thead")]
    Thead,
    #[parse(rename = "<time")]
    Time,
    #[parse(rename = "<title")]
    Title,
    #[parse(rename = "<tr")]
    Tr,
    #[parse(rename = "<track")]
    Track,
    #[parse(rename = "<u")]
    U,
    #[parse(rename = "<ul")]
    Ul,
    #[parse(rename = "<var")]
    Var,
    #[parse(rename = "<video")]
    Video,
    #[parse(rename = "<wbr")]
    Wbr,
}
#[derive(Debug, Clone)]
pub enum Element {
    A(A),
    Abbr(Abbr),
    Address(Address),
    Area(Area),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    B(B),
    Base(Base),
    Bdi(Bdi),
    Bdo(Bdo),
    Blockquote(Blockquote),
    Body(Body),
    Br(Br),
    Button(Button),
    Canvas(Canvas),
    Caption(Caption),
    Cite(Cite),
    Code(Code),
    Col(Col),
    Colgroup(Colgroup),
    Data(Data),
    Datalist(Datalist),
    Dd(Dd),
    Del(Del),
    Details(Details),
    Dfn(Dfn),
    Dialog(Dialog),
    Div(Div),
    Dl(Dl),
    Dt(Dt),
    Em(Em),
    Embed(Embed),
    Fieldset(Fieldset),
    Figcaption(Figcaption),
    Figure(Figure),
    Footer(Footer),
    Form(Form),
    H1(H1),
    H2(H2),
    H3(H3),
    H4(H4),
    H5(H5),
    H6(H6),
    Head(Head),
    Header(Header),
    Hgroup(Hgroup),
    Hr(Hr),
    Html(Html),
    I(I),
    Iframe(Iframe),
    Img(Img),
    Input(Input),
    Ins(Ins),
    Kbd(Kbd),
    Label(Label),
    Legend(Legend),
    Li(Li),
    Link(Link),
    Main(Main),
    Map(Map),
    Mark(Mark),
    Menu(Menu),
    Meta(Meta),
    Meter(Meter),
    Nav(Nav),
    Noscript(Noscript),
    Object(Object),
    Ol(Ol),
    Optgroup(Optgroup),
    Option(Option),
    Output(Output),
    P(P),
    Param(Param),
    Picture(Picture),
    Pre(Pre),
    Progress(Progress),
    Q(Q),
    Rb(Rb),
    Rp(Rp),
    Rt(Rt),
    Ruby(Ruby),
    S(S),
    Samp(Samp),
    Script(Script),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Source(Source),
    Span(Span),
    Strong(Strong),
    Style(Style),
    Sub(Sub),
    Summary(Summary),
    Sup(Sup),
    Table(Table),
    Tbody(Tbody),
    Td(Td),
    Template(Template),
    Textarea(Textarea),
    Tfoot(Tfoot),
    Th(Th),
    Thead(Thead),
    Time(Time),
    Title(Title),
    Tr(Tr),
    Track(Track),
    U(U),
    Ul(Ul),
    Var(Var),
    Video(Video),
    Wbr(Wbr),
}
impl kalosm_sample::Parse for Element {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ElementName::new_parser()
            .then_lazy(|name| match name {
                ElementName::A => A::new_parser().map_output(Self::A).boxed(),
                ElementName::Abbr => Abbr::new_parser().map_output(Self::Abbr).boxed(),
                ElementName::Address => Address::new_parser().map_output(Self::Address).boxed(),
                ElementName::Area => Area::new_parser().map_output(Self::Area).boxed(),
                ElementName::Article => Article::new_parser().map_output(Self::Article).boxed(),
                ElementName::Aside => Aside::new_parser().map_output(Self::Aside).boxed(),
                ElementName::Audio => Audio::new_parser().map_output(Self::Audio).boxed(),
                ElementName::B => B::new_parser().map_output(Self::B).boxed(),
                ElementName::Base => Base::new_parser().map_output(Self::Base).boxed(),
                ElementName::Bdi => Bdi::new_parser().map_output(Self::Bdi).boxed(),
                ElementName::Bdo => Bdo::new_parser().map_output(Self::Bdo).boxed(),
                ElementName::Blockquote => Blockquote::new_parser()
                    .map_output(Self::Blockquote)
                    .boxed(),
                ElementName::Body => Body::new_parser().map_output(Self::Body).boxed(),
                ElementName::Br => Br::new_parser().map_output(Self::Br).boxed(),
                ElementName::Button => Button::new_parser().map_output(Self::Button).boxed(),
                ElementName::Canvas => Canvas::new_parser().map_output(Self::Canvas).boxed(),
                ElementName::Caption => Caption::new_parser().map_output(Self::Caption).boxed(),
                ElementName::Cite => Cite::new_parser().map_output(Self::Cite).boxed(),
                ElementName::Code => Code::new_parser().map_output(Self::Code).boxed(),
                ElementName::Col => Col::new_parser().map_output(Self::Col).boxed(),
                ElementName::Colgroup => Colgroup::new_parser().map_output(Self::Colgroup).boxed(),
                ElementName::Data => Data::new_parser().map_output(Self::Data).boxed(),
                ElementName::Datalist => Datalist::new_parser().map_output(Self::Datalist).boxed(),
                ElementName::Dd => Dd::new_parser().map_output(Self::Dd).boxed(),
                ElementName::Del => Del::new_parser().map_output(Self::Del).boxed(),
                ElementName::Details => Details::new_parser().map_output(Self::Details).boxed(),
                ElementName::Dfn => Dfn::new_parser().map_output(Self::Dfn).boxed(),
                ElementName::Dialog => Dialog::new_parser().map_output(Self::Dialog).boxed(),
                ElementName::Div => Div::new_parser().map_output(Self::Div).boxed(),
                ElementName::Dl => Dl::new_parser().map_output(Self::Dl).boxed(),
                ElementName::Dt => Dt::new_parser().map_output(Self::Dt).boxed(),
                ElementName::Em => Em::new_parser().map_output(Self::Em).boxed(),
                ElementName::Embed => Embed::new_parser().map_output(Self::Embed).boxed(),
                ElementName::Fieldset => Fieldset::new_parser().map_output(Self::Fieldset).boxed(),
                ElementName::Figcaption => Figcaption::new_parser()
                    .map_output(Self::Figcaption)
                    .boxed(),
                ElementName::Figure => Figure::new_parser().map_output(Self::Figure).boxed(),
                ElementName::Footer => Footer::new_parser().map_output(Self::Footer).boxed(),
                ElementName::Form => Form::new_parser().map_output(Self::Form).boxed(),
                ElementName::H1 => H1::new_parser().map_output(Self::H1).boxed(),
                ElementName::H2 => H2::new_parser().map_output(Self::H2).boxed(),
                ElementName::H3 => H3::new_parser().map_output(Self::H3).boxed(),
                ElementName::H4 => H4::new_parser().map_output(Self::H4).boxed(),
                ElementName::H5 => H5::new_parser().map_output(Self::H5).boxed(),
                ElementName::H6 => H6::new_parser().map_output(Self::H6).boxed(),
                ElementName::Head => Head::new_parser().map_output(Self::Head).boxed(),
                ElementName::Header => Header::new_parser().map_output(Self::Header).boxed(),
                ElementName::Hgroup => Hgroup::new_parser().map_output(Self::Hgroup).boxed(),
                ElementName::Hr => Hr::new_parser().map_output(Self::Hr).boxed(),
                ElementName::Html => Html::new_parser().map_output(Self::Html).boxed(),
                ElementName::I => I::new_parser().map_output(Self::I).boxed(),
                ElementName::Iframe => Iframe::new_parser().map_output(Self::Iframe).boxed(),
                ElementName::Img => Img::new_parser().map_output(Self::Img).boxed(),
                ElementName::Input => Input::new_parser().map_output(Self::Input).boxed(),
                ElementName::Ins => Ins::new_parser().map_output(Self::Ins).boxed(),
                ElementName::Kbd => Kbd::new_parser().map_output(Self::Kbd).boxed(),
                ElementName::Label => Label::new_parser().map_output(Self::Label).boxed(),
                ElementName::Legend => Legend::new_parser().map_output(Self::Legend).boxed(),
                ElementName::Li => Li::new_parser().map_output(Self::Li).boxed(),
                ElementName::Link => Link::new_parser().map_output(Self::Link).boxed(),
                ElementName::Main => Main::new_parser().map_output(Self::Main).boxed(),
                ElementName::Map => Map::new_parser().map_output(Self::Map).boxed(),
                ElementName::Mark => Mark::new_parser().map_output(Self::Mark).boxed(),
                ElementName::Menu => Menu::new_parser().map_output(Self::Menu).boxed(),
                ElementName::Meta => Meta::new_parser().map_output(Self::Meta).boxed(),
                ElementName::Meter => Meter::new_parser().map_output(Self::Meter).boxed(),
                ElementName::Nav => Nav::new_parser().map_output(Self::Nav).boxed(),
                ElementName::Noscript => Noscript::new_parser().map_output(Self::Noscript).boxed(),
                ElementName::Object => Object::new_parser().map_output(Self::Object).boxed(),
                ElementName::Ol => Ol::new_parser().map_output(Self::Ol).boxed(),
                ElementName::Optgroup => Optgroup::new_parser().map_output(Self::Optgroup).boxed(),
                ElementName::Option => Option::new_parser().map_output(Self::Option).boxed(),
                ElementName::Output => Output::new_parser().map_output(Self::Output).boxed(),
                ElementName::P => P::new_parser().map_output(Self::P).boxed(),
                ElementName::Param => Param::new_parser().map_output(Self::Param).boxed(),
                ElementName::Picture => Picture::new_parser().map_output(Self::Picture).boxed(),
                ElementName::Pre => Pre::new_parser().map_output(Self::Pre).boxed(),
                ElementName::Progress => Progress::new_parser().map_output(Self::Progress).boxed(),
                ElementName::Q => Q::new_parser().map_output(Self::Q).boxed(),
                ElementName::Rb => Rb::new_parser().map_output(Self::Rb).boxed(),
                ElementName::Rp => Rp::new_parser().map_output(Self::Rp).boxed(),
                ElementName::Rt => Rt::new_parser().map_output(Self::Rt).boxed(),
                ElementName::Ruby => Ruby::new_parser().map_output(Self::Ruby).boxed(),
                ElementName::S => S::new_parser().map_output(Self::S).boxed(),
                ElementName::Samp => Samp::new_parser().map_output(Self::Samp).boxed(),
                ElementName::Script => Script::new_parser().map_output(Self::Script).boxed(),
                ElementName::Section => Section::new_parser().map_output(Self::Section).boxed(),
                ElementName::Select => Select::new_parser().map_output(Self::Select).boxed(),
                ElementName::Slot => Slot::new_parser().map_output(Self::Slot).boxed(),
                ElementName::Small => Small::new_parser().map_output(Self::Small).boxed(),
                ElementName::Source => Source::new_parser().map_output(Self::Source).boxed(),
                ElementName::Span => Span::new_parser().map_output(Self::Span).boxed(),
                ElementName::Strong => Strong::new_parser().map_output(Self::Strong).boxed(),
                ElementName::Style => Style::new_parser().map_output(Self::Style).boxed(),
                ElementName::Sub => Sub::new_parser().map_output(Self::Sub).boxed(),
                ElementName::Summary => Summary::new_parser().map_output(Self::Summary).boxed(),
                ElementName::Sup => Sup::new_parser().map_output(Self::Sup).boxed(),
                ElementName::Table => Table::new_parser().map_output(Self::Table).boxed(),
                ElementName::Tbody => Tbody::new_parser().map_output(Self::Tbody).boxed(),
                ElementName::Td => Td::new_parser().map_output(Self::Td).boxed(),
                ElementName::Template => Template::new_parser().map_output(Self::Template).boxed(),
                ElementName::Textarea => Textarea::new_parser().map_output(Self::Textarea).boxed(),
                ElementName::Tfoot => Tfoot::new_parser().map_output(Self::Tfoot).boxed(),
                ElementName::Th => Th::new_parser().map_output(Self::Th).boxed(),
                ElementName::Thead => Thead::new_parser().map_output(Self::Thead).boxed(),
                ElementName::Time => Time::new_parser().map_output(Self::Time).boxed(),
                ElementName::Title => Title::new_parser().map_output(Self::Title).boxed(),
                ElementName::Tr => Tr::new_parser().map_output(Self::Tr).boxed(),
                ElementName::Track => Track::new_parser().map_output(Self::Track).boxed(),
                ElementName::U => U::new_parser().map_output(Self::U).boxed(),
                ElementName::Ul => Ul::new_parser().map_output(Self::Ul).boxed(),
                ElementName::Var => Var::new_parser().map_output(Self::Var).boxed(),
                ElementName::Video => Video::new_parser().map_output(Self::Video).boxed(),
                ElementName::Wbr => Wbr::new_parser().map_output(Self::Wbr).boxed(),
            })
            .map_output(|(_, element)| element)
    }
}
