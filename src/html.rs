#![allow(clippy::enum_variant_names)]
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
