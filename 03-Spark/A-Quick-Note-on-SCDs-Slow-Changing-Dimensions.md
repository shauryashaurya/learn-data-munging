The *correct* SCD type often depends heavily on the specific **business requirements** for historical tracking and analysis.            
An attribute might be Type 1 in one company and Type 2 in another.            
          
-----          
          
##  SCD Type 0: Fixed / Retain Original          
          
  * **Rule:** The value is assigned once and never changed. If the source value changes, the change is ignored in the dimension.          
  * **Examples:**          
    1.  **`DateOfBirth` (Customer/Employee):** Physiologically fixed. Corrections might happen (making it Type 1 in practice for errors), but conceptually, it *should* be fixed.          
    2.  **`OriginalHireDate` (Employee):** The date the employee first joined the company, regardless of re-hires or leaves.          
    3.  **`ProductCreationTimestamp`:** The exact time a product record was first created in the source system.          
    4.  **`AccountOpenedDate` (Customer):** The date a customer first opened their account.          
    5.  **`PublicationDate` (Book Dimension):** The official first publication date of a specific edition.          
    6.  **`SurveyTakenDate`:** The date a specific instance of a survey was completed by a respondent.          
    7.  **`OrderPlacedTimestamp`:** The immutable timestamp when an order was finalized.          
    -----          
    8.  **`InitialDiagnosisCode` (Patient) - *Harder to Identify*:** If the requirement is to *always* keep the very first diagnosis code assigned, even if it's later refined or corrected, this specific attribute representing the *initial* code could be Type 0. The challenge is confirming the business rule strictly requires preserving the potentially incorrect *initial* code rather than correcting it (Type 1) or tracking the changes (Type 2).          
    9.  **`FirstWebsiteVisitSource` (Customer) - *Harder to Identify*:** The marketing channel (e.g., 'Organic Search', 'Paid Ad', 'Referral') that led to the customer's *very first* recorded interaction. While potentially subject to data capture errors, the business might decide this initial source attribution is immutable for cohort analysis, making it Type 0. Ambiguity arises if corrections *are* allowed.          
    10. **`ImmutableGeographicRegionID` (e.g., Country Code) - *Harder to Identify*:** A country code like 'US' or 'IN' seems fixed. However, geopolitical changes, though rare, can happen (country splits/merges). If the dimension represents *current* geopolitical entities, it might need Type 2. If the attribute represents the region *as defined at the time the record was created* and is never backdated, it acts as Type 0 for that specific record's context, even if the real-world entity changes later. The definition of the attribute is key.          
          
-----          
          
##  SCD Type 1: Overwrite          
          
  * **Rule:** The old value is replaced with the new value. No history is kept for the attribute.          
  * **Examples:**          
    1.  **`CustomerEmailAddress` (Correction):** Fixing a typo in an email ('[email address removed]' -\> '[email address removed]'). History of the typo isn't useful.          
    2.  **`ProductDescription` (Minor Update):** Updating a product description to add more detail or fix grammar, where past descriptions aren't needed for analysis.          
    3.  **`EmployeeWorkPhoneNumber`:** History of past phone numbers usually isn't required for typical business analysis.          
    4.  **`WebPageURL` (Correction):** Fixing a broken link URL stored in a dimension, assuming the old broken URL isn't needed.          
    5.  **`CustomerMiddleName`:** Often only the current value is stored; historical middle names (e.g., pre-marriage) are rarely tracked unless legally required.          
    6.  **`ProductIsActiveFlag`:** A boolean flag indicating if a product is currently sold. Past status might be tracked via sales dates, not necessarily historical changes to this specific flag, if only current active products are queried.          
    7.  **`UserInterfaceThemePreference`:** A user's currently selected theme (e.g., 'dark', 'light'). Past preferences are usually irrelevant.          
    -----          
    8.  **`ProductCategoryName` (Simple Re-categorization) - *Harder to Identify*:** If a product 'Widget A' is moved from 'Gadgets' to 'Super Gadgets', and the business *only* cares about reporting sales based on the *current* category structure, overwriting 'Gadgets' with 'Super Gadgets' (Type 1) is simplest. However, if analyzing historical sales trends *requires* knowing it was sold as a 'Gadget' previously, then Type 2 would be needed. The analytical requirement makes the choice difficult.          
    9.  **`Customer's Preferred Store` - *Harder to Identify*:** A customer might change their preferred store location. If analysis only focuses on linking current customers to their *current* preferred store, Type 1 works. If analyzing purchase patterns based on which store was preferred *at the time of purchase* matters, Type 2 is necessary.          
    10. **`Bug Status` (e.g., 'Open', 'In Progress', 'Resolved') - *Harder to Identify*:** In a bug tracking dimension, if you only report on the *current* status of bugs, Type 1 seems appropriate. However, the *history* of status changes (when was it opened, worked on, resolved) is usually critical. This pushes towards Type 2 for a 'BugStatusHistory' table or tracking status changes in a related fact/event table, making the simple 'CurrentBugStatus' attribute potentially Type 1 *if used carefully alongside other history*.          
          
-----          
          
##  SCD Type 2: Add New Row / Row Versioning          
          
  * **Rule:** Track full attribute history by expiring the old row (setting an end date/flag) and inserting a new row with the updated value(s) and a new effective start date. Requires surrogate keys.          
  * **Examples:**          
    1.  **`CustomerAddress`:** Essential for knowing where orders were shipped historically and for geographical analysis over time.          
    2.  **`EmployeeDepartment` or `EmployeeJobTitle`:** Crucial for tracking career progression, organizational structure changes, and historical reporting.          
    3.  **`ProductPrice` or `ProductCost`:** Necessary for calculating historical profit margins, analyzing price elasticity, and auditing.          
    4.  **`SalespersonTerritoryAssignment`:** Tracking which salesperson covered which territory during specific periods is vital for sales performance reporting.          
    5.  **`MaritalStatus` (Customer/Employee):** Changes impact demographic analysis and potentially benefits administration.          
    6.  **`ProductHierarchyLevel` (e.g., Category, SubCategory):** If the hierarchy structure itself changes, tracking historical structures is needed for consistent reporting over time.          
    7.  **`CustomerCreditLimit`:** History is vital for risk analysis and understanding credit behavior over time.          
    -----          
    8.  **`SubscriptionPlanLevel` (e.g., Free, Basic, Pro) - *Harder to Identify Implementation*:** Clearly needs history (Type 2). The challenge lies in the *frequency* and *timing* of changes. If users can switch plans frequently, the ETL process must run often enough to capture these changes accurately with correct start/end dates. Missed intermediate states (e.g., Pro -\> Basic -\> Pro between ETL runs) can occur, making perfect Type 2 hard.          
    9.  **`GeopoliticalBoundary` (e.g., Sales Region Definition) - *Harder to Identify Scope*:** A sales region 'NorthEast' might be redefined to include/exclude certain states/counties. Tracking the *definition* change requires Type 2 on the region dimension. The challenge is ensuring all related facts (sales, customers) correctly link to the territory version that was active *at the time the fact occurred*. Linking logic can be complex.          
    10. **`Relationship Manager Assignment` (Bank Customer) - *Harder to Identify Granularity*:** Tracking which manager was assigned to a customer seems like straightforward Type 2. The challenge arises if multiple *types* of relationships exist (e.g., Wealth Manager, Mortgage Advisor) or if assignments change very frequently. Should there be one Type 2 dimension tracking the primary manager, or separate tracking for different roles? How often must it be updated?          
          
-----          
          
##  SCD Type 3: Add New Column / Previous Value          
          
  * **Rule:** Track limited history by adding a "previous value" column alongside the current value. Only the immediate prior state is kept.          
  * **Examples:**          
    1.  **`EmployeePreviousJobTitle`:** Alongside `CurrentJobTitle`.          
    2.  **`ProductPreviousPrice`:** Alongside `CurrentPrice`.          
    3.  **`StockLocationPreviousAisle`:** Alongside `CurrentAisle` for quick reference to recent moves.          
    4.  **`CustomerPreviousState`:** Alongside `CurrentState` (limited usefulness compared to Type 2 address).          
    5.  **`WebsitePreviousHomePageLayoutID`:** Alongside `CurrentHomePageLayoutID` for quick A/B comparison reference.          
    6.  **`TaskPreviousAssignee`:** Alongside `CurrentAssignee`.          
    7.  **`SoftwarePreviousVersionNumber`:** Alongside `CurrentVersionNumber` deployed on a server.          
    -----          
    8.  **`CoursePreviousPrerequisite` - *Harder to Identify Sufficiency*:** Tracking the immediately prior prerequisite for a university course might seem useful. However, prerequisite chains can be long and complex. Type 3 only gives one step back, which might be insufficient for analyzing curriculum evolution or student pathways. Choosing Type 3 implies this limited view is *enough*.          
    9.  **`SupplierPreviousRating` - *Harder to Identify Temporal Context*:** Storing the last rating alongside the current one gives a delta. But *when* was that previous rating assigned? Type 3 often lacks the timestamp context associated with the previous value, limiting its analytical depth compared to Type 2 which tracks effective dates.          
    10. **`CampaignPreviousTargetAudienceSegment` - *Harder to Identify Value*:** Knowing the immediately prior audience segment might help understand a recent campaign shift. But without the full history of audience targeting changes (Type 2), analyzing long-term campaign strategy evolution is difficult. Is the limited Type 3 view genuinely valuable?          
          
-----          
          
##  SCD Type 4: Mini-Dimension          
          
  * **Rule:** A group of attributes that change together or frequently are moved to a separate dimension table. The main dimension table holds a foreign key to the current mini-dimension record. History is often tracked within the mini-dimension itself (usually using Type 2).          
  * **Examples:**          
    1.  **`DimDemographics`:** Grouping customer attributes like `AgeBand`, `IncomeBucket`, `EducationLevel`, `MaritalStatus` that might be updated together from survey data. `DimCustomer` holds `current_demographic_sk`.          
    2.  **`DimProductProperties`:** For electronics, grouping `RAMSize`, `StorageType`, `ScreenResolution` into a configuration profile. `DimProduct` holds `current_properties_sk`.          
    3.  **`DimEmployeeCompensationBand`:** Grouping `SalaryGrade`, `BonusTargetPercent`, `StockOptionLevel`. `DimEmployee` holds `current_comp_band_sk`.          
    4.  **`DimWebSessionDetails`:** Grouping `Browser`, `OperatingSystem`, `DeviceType` derived from a user agent string. `DimWebLog` might hold `session_details_sk`.          
    5.  **`DimStoreLayoutProfile`:** Describing the layout configuration of a retail store (e.g., `NumCheckouts`, `AisleConfigurationCode`, `SqFootageBand`). `DimStore` holds `current_layout_profile_sk`.          
    6.  **`DimPatientRiskScoreProfile`:** Combining multiple calculated risk scores (e.g., `CardiovascularRiskScore`, `DiabetesRiskScore`, `FallRiskIndicator`). `DimPatient` holds `current_risk_profile_sk`.          
    7.  **`DimClimateMeasurementProfile`:** For weather stations, grouping related measurements like `TemperatureBand`, `HumidityLevel`, `PressureTrendIndicator`. `DimWeatherReadingFact` might hold `climate_profile_sk`.          
    -----          
    8.  **`SurveyResponseGroup` - *Harder to Identify Structure*:** If a customer answers a 10-question survey multiple times over years, each set of 10 answers represents a distinct profile. These 10 answers can form a mini-dimension row (`DimSurveyResponse`), linked to the customer survey event fact. The challenge is recognizing that these separate answers logically form a single "profile" that should be dimensionalized together.          
    9.  **`ServerConfigurationSnapshot` - *Harder to Identify Boundary*:** Grouping `OSVersion`, `PatchLevel`, `CPUCoreCount`, `MemoryAssigned` for servers. The challenge is deciding *which* attributes belong in the mini-dimension versus staying on `DimServer` or being handled differently. If only OS/Patch change frequently *together*, maybe only they form the mini-dimension.          
    10. **`CustomerBehavioralSegmentProfile` - *Harder to Identify Stability*:** Grouping calculated behavioral flags like `IsFrequentBuyer`, `IsWeekendShopper`, `PrefersDiscount`, `ChurnRiskLevel`. These flags might be recalculated daily or weekly. The challenge is that the *definition* of the segment itself might change (SCD Type 2 within the mini-dim), and the customer's assignment also changes frequently. Is a mini-dimension better than just tracking segment assignments over time directly against the customer? Depends on the number of attributes and analysis needs.          
          
-----          
          
##  SCD Type 6: Hybrid (Type 2 + Type 1/3)          
          
  * **Rule:** Combines Type 2 (full history via rows/SKs/dates) with updates to current value columns across historical rows (like Type 1/3) for ease of reporting current state contextually with history.          
  * **Examples:**          
    1.  **`DimEmployee`:** Type 2 history for `DepartmentID`, `StartDate`, `EndDate`, plus a `CurrentDepartmentName` column updated on all employee rows.          
    2.  **`DimProduct`:** Type 2 history for `Price`, `PriceStartDate`, `PriceEndDate`, plus a `CurrentPrice` column updated on all product rows.          
    3.  **`DimCustomer`:** Type 2 history for `Address`, `AddrStartDate`, `AddrEndDate`, plus `CurrentState` and `CurrentZipCode` columns updated on all customer rows.          
    4.  **`DimSalesTerritory`:** Type 2 history for `RegionalManagerID`, `AssignStartDate`, `AssignEndDate`, plus a `CurrentManagerName` column.          
    5.  **`DimSubscription`:** Type 2 history for `PlanLevel`, `PlanStartDate`, `PlanEndDate`, plus `CurrentPlanLevel` and `IsCurrentlyActive` columns.          
    6.  **`DimStore`:** Type 2 history for `StoreManagerID`, `MgrStartDate`, `MgrEndDate`, plus `CurrentStoreManagerName`.          
    7.  **`DimProject`:** Type 2 history for `ProjectStatus` (e.g., Green, Amber, Red), `StatusStartDate`, `StatusEndDate`, plus `CurrentProjectStatus` column.          
    -----          
    8.  **`DimPortfolio` (Financial) - *Harder to Implement Update*:** Tracking the composition (holdings, weights - Type 2) over time, but also including `CurrentPortfolioMarketValue` and `CurrentRiskRating` updated across all historical versions. The challenge is the potentially high frequency of updates needed for the 'Current' fields and ensuring consistency during the ETL update process across many historical rows.          
    9.  **`DimNetworkDevice` - *Harder to Define 'Current'*:** Tracking firmware versions using Type 2 (`FirmwareVersion`, `InstallDate`, `EndDate`), but also including a `CurrentFirmwareVersion` column. If devices can have firmware rolled back, the definition of 'Current' needs care. Does it reflect the *latest installed* regardless of success, or the *latest known stable/operational* version?          
    10. **`DimClinicalTrialPatient` - *Harder to Justify Complexity*:** Tracking patient status within a trial (e.g., 'Screening', 'Enrolled', 'OnTreatment', 'FollowUp', 'Completed' - Type 2) but adding `CurrentStatus` and maybe `CurrentTreatmentArm` columns. While possible, the added complexity of Type 6 updates needs strong justification versus just querying the Type 2 history effectively using the `is_current` flag or date ranges. Is the reporting simplification worth the ETL complexity?