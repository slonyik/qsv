use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn diff_sort_diff_result_on_first_column_with_qsv_sort_cmd() {
    let wrk = Workdir::new("diff");
    let test_file = wrk.load_test_file("boston311-100.csv");
    let test_file2 = wrk.load_test_file("boston311-100-diff.csv");

    let mut cmd = wrk.command("diff");
    cmd.arg(test_file).arg(test_file2);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // sort on the 1st column, case_enquiry_id
    // --select is set to 2 coz `diff` prepends
    // a "diffresult" column
    let mut cmd = wrk.command("sort");
    cmd.arg("--select").arg("2").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 =
        expected_diff_result_sort_on_first_column_original_is_left_arg_and_diff_is_right_arg();

    assert_eq!(dos2unix(&got2), dos2unix(&expected2));

    fn expected_diff_result_sort_on_first_column_original_is_left_arg_and_diff_is_right_arg(
    ) -> String {
        r#"
diffresult,case_enquiry_id,open_dt,target_dt,closed_dt,ontime,case_status,closure_reason,case_title,subject,reason,type,queue,department,submittedphoto,closedphoto,location,fire_district,pwd_district,city_council_district,police_district,neighborhood,neighborhood_services_district,ward,precinct,location_street_name,location_zipcode,latitude,longitude,source
-,101004113747,2022-01-01 23:46:09,2022-01-17 08:30:00,2022-01-02 11:03:10,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 11:03:10 EST 2022 Noted Case noted. Duplicate case. Posts already marked for contractor to repair.  ,Street Light Outages,Public Works Department,Street Lights,Street Light Outages,PWDx_Street Light Outages,PWDx,https://311.boston.gov/media/boston/report/photos/61d12e0705bbcf180c29cfc2/report.jpg,,103 N Beacon St  Brighton  MA  02135,11,04,9,D14,Brighton,15,22,2205,103 N Beacon St,02135,42.3549,-71.143,Citizens Connect App
+,101004113747,2022-01-01 23:46:09,2022-01-17 08:30:00,2022-01-02 11:04:10,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 11:03:10 EST 2022 Noted Case noted. Duplicate case. Posts already marked for contractor to repair.  ,Street Light Outages,Public Works Department,Street Lights,Street Light Outages,PWDx_Street Light Outages,PWDx,https://311.boston.gov/media/boston/report/photos/61d12e0705bbcf180c29cfc2/report.jpg,,103 N Beacon St  Brighton  MA  02135,11,04,9,D14,Brighton,15,22,2205,103 N Beacon St,02135,42.3549,-71.143,Citizens Connect App
-,101004114069,2022-01-02 14:11:49,2022-01-05 08:30:00,2022-01-03 06:52:40,ONTIME,Closed,Case Closed. Closed date : Mon Jan 03 06:52:40 EST 2022 Resolved No violation found at this time  today is trash day.  ,Improper Storage of Trash (Barrels),Public Works Department,Code Enforcement,Improper Storage of Trash (Barrels),PWDx_Code Enforcement,PWDx,https://311.boston.gov/media/boston/report/photos/61d1f8e905bbcf180c2a3d7f/report.jpg,,22 Henchman St  Boston  MA  02109,3,1B,1,A1,Downtown / Financial District,3,Ward 3,0302,22 Henchman St,02109,42.3674,-71.0537,Citizens Connect App
+,101004114069,2022-01-02 14:11:49,2022-01-05 08:30:00,2022-01-03 06:52:40,ONTIME,Closed,Case Closed. Closed date : Mon Jan 03 06:52:40 EST 2022 Resolved No violation found at this time today is trash day.  ,Improper Storage of Trash (Barrels),Public Works Department,Code Enforcement,Improper Storage of Trash (Barrels),PWDx_Code Enforcement,PWDx,https://311.boston.gov/media/boston/report/photos/61d1f8e905bbcf180c2a3d7f/report.jpg,,22 Henchman St  Boston  MA  02109,3,1B,1,A1,Downtown / Financial District,3,Ward 3,0302,22 Henchman St,02109,42.3674,-71.0537,Citizens Connect App
-,101004114152,2022-01-02 16:18:30,2022-01-10 08:30:00,2022-01-02 16:32:54,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 16:32:54 EST 2022 Noted This not not a city park  ,Litter / Ground Maintenance - Wellington Green (BPRD),Parks & Recreation Department,Park Maintenance & Safety,Ground Maintenance,PARK_Maintenance_Ground Maintenance,PARK,https://311.boston.gov/media/boston/report/photos/61d2169605bbcf180c2a4d65/photo_20220102_161627.jpg,,563 Columbus Ave  Roxbury  MA  02118,4,1C,7,D4,South End,6,Ward 4,0404,563 Columbus Ave,02118,42.3412,-71.0815,Citizens Connect App
+,101004114152,2022-01-02 16:18:30,2022-01-10 08:30:00,2022-01-02 16:32:54,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 16:32:54 EST 2022 Noted This not not a city park  ,Litter/Ground Maintenance - Wellington Green (BPRD),Parks & Recreation Department,Park Maintenance & Safety,Ground Maintenance,PARK_Maintenance_Ground Maintenance,PARK,https://311.boston.gov/media/boston/report/photos/61d2169605bbcf180c2a4d65/photo_20220102_161627.jpg,,563 Columbus Ave  Roxbury  MA  02118,4,1C,7,D4,South End,6,Ward 4,0404,563 Columbus Ave,02118,42.3412,-71.0815,Citizens Connect App
-,101004114377,2022-01-03 07:50:09,2022-01-04 08:30:00,2022-01-03 10:35:57,ONTIME,Closed,Case Closed. Closed date : 2022-01-03 10:35:57.797 Case Resolved Vehicles mere moved will check again  ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,,,618 E Sixth St  South Boston  MA  02127,6,05,2,C6,South Boston / South Boston Waterfront,5,Ward 6,0606,618 E Sixth St,02127,42.3332,-71.0357,Citizens Connect App
+,101004114377,2022-01-03 07:50:09,2022-01-04 08:30:00,2022-01-03 10:35:57,ONTIME,Closed,Case Closed. Closed date : 2022-01-03 10:35:57.797 Case Resolved Vehicles mere moved will check again sir ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,,,618 E Sixth St  South Boston  MA  02127,6,05,2,C6,South Boston / South Boston Waterfront,5,Ward 6,0606,618 E Sixth St,02127,42.3332,-71.0357,Citizens Connect App
        "#.trim().to_string()
    }
}

#[test]
fn diff_original_left_and_diff_right_sort_diff_result_by_lines_by_default() {
    let wrk = Workdir::new("diff");
    let test_file = wrk.load_test_file("boston311-100.csv");
    let test_file2 = wrk.load_test_file("boston311-100-diff.csv");

    let mut cmd = wrk.command("diff");
    cmd.arg(test_file).arg(test_file2);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let diff_result_file_name = "diff_result_original_left_diff_right.csv";

    wrk.create(diff_result_file_name, got);

    let mut cmd = wrk.command("select");
    // select all columns
    cmd.arg("1-").arg(diff_result_file_name);

    let actual: String = wrk.stdout(&mut cmd);
    let expected = create_expected_diff_result_when_sorting_by_lines_original_is_left_arg_and_diff_is_right_arg();

    assert_eq!(dos2unix(&actual), dos2unix(&expected));

    fn create_expected_diff_result_when_sorting_by_lines_original_is_left_arg_and_diff_is_right_arg(
    ) -> String {
        r#"
diffresult,case_enquiry_id,open_dt,target_dt,closed_dt,ontime,case_status,closure_reason,case_title,subject,reason,type,queue,department,submittedphoto,closedphoto,location,fire_district,pwd_district,city_council_district,police_district,neighborhood,neighborhood_services_district,ward,precinct,location_street_name,location_zipcode,latitude,longitude,source
-,101004113747,2022-01-01 23:46:09,2022-01-17 08:30:00,2022-01-02 11:03:10,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 11:03:10 EST 2022 Noted Case noted. Duplicate case. Posts already marked for contractor to repair.  ,Street Light Outages,Public Works Department,Street Lights,Street Light Outages,PWDx_Street Light Outages,PWDx,https://311.boston.gov/media/boston/report/photos/61d12e0705bbcf180c29cfc2/report.jpg,,103 N Beacon St  Brighton  MA  02135,11,04,9,D14,Brighton,15,22,2205,103 N Beacon St,02135,42.3549,-71.143,Citizens Connect App
+,101004113747,2022-01-01 23:46:09,2022-01-17 08:30:00,2022-01-02 11:04:10,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 11:03:10 EST 2022 Noted Case noted. Duplicate case. Posts already marked for contractor to repair.  ,Street Light Outages,Public Works Department,Street Lights,Street Light Outages,PWDx_Street Light Outages,PWDx,https://311.boston.gov/media/boston/report/photos/61d12e0705bbcf180c29cfc2/report.jpg,,103 N Beacon St  Brighton  MA  02135,11,04,9,D14,Brighton,15,22,2205,103 N Beacon St,02135,42.3549,-71.143,Citizens Connect App
-,101004114377,2022-01-03 07:50:09,2022-01-04 08:30:00,2022-01-03 10:35:57,ONTIME,Closed,Case Closed. Closed date : 2022-01-03 10:35:57.797 Case Resolved Vehicles mere moved will check again  ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,,,618 E Sixth St  South Boston  MA  02127,6,05,2,C6,South Boston / South Boston Waterfront,5,Ward 6,0606,618 E Sixth St,02127,42.3332,-71.0357,Citizens Connect App
+,101004114377,2022-01-03 07:50:09,2022-01-04 08:30:00,2022-01-03 10:35:57,ONTIME,Closed,Case Closed. Closed date : 2022-01-03 10:35:57.797 Case Resolved Vehicles mere moved will check again sir ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,,,618 E Sixth St  South Boston  MA  02127,6,05,2,C6,South Boston / South Boston Waterfront,5,Ward 6,0606,618 E Sixth St,02127,42.3332,-71.0357,Citizens Connect App
-,101004114069,2022-01-02 14:11:49,2022-01-05 08:30:00,2022-01-03 06:52:40,ONTIME,Closed,Case Closed. Closed date : Mon Jan 03 06:52:40 EST 2022 Resolved No violation found at this time  today is trash day.  ,Improper Storage of Trash (Barrels),Public Works Department,Code Enforcement,Improper Storage of Trash (Barrels),PWDx_Code Enforcement,PWDx,https://311.boston.gov/media/boston/report/photos/61d1f8e905bbcf180c2a3d7f/report.jpg,,22 Henchman St  Boston  MA  02109,3,1B,1,A1,Downtown / Financial District,3,Ward 3,0302,22 Henchman St,02109,42.3674,-71.0537,Citizens Connect App
+,101004114069,2022-01-02 14:11:49,2022-01-05 08:30:00,2022-01-03 06:52:40,ONTIME,Closed,Case Closed. Closed date : Mon Jan 03 06:52:40 EST 2022 Resolved No violation found at this time today is trash day.  ,Improper Storage of Trash (Barrels),Public Works Department,Code Enforcement,Improper Storage of Trash (Barrels),PWDx_Code Enforcement,PWDx,https://311.boston.gov/media/boston/report/photos/61d1f8e905bbcf180c2a3d7f/report.jpg,,22 Henchman St  Boston  MA  02109,3,1B,1,A1,Downtown / Financial District,3,Ward 3,0302,22 Henchman St,02109,42.3674,-71.0537,Citizens Connect App
-,101004114152,2022-01-02 16:18:30,2022-01-10 08:30:00,2022-01-02 16:32:54,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 16:32:54 EST 2022 Noted This not not a city park  ,Litter / Ground Maintenance - Wellington Green (BPRD),Parks & Recreation Department,Park Maintenance & Safety,Ground Maintenance,PARK_Maintenance_Ground Maintenance,PARK,https://311.boston.gov/media/boston/report/photos/61d2169605bbcf180c2a4d65/photo_20220102_161627.jpg,,563 Columbus Ave  Roxbury  MA  02118,4,1C,7,D4,South End,6,Ward 4,0404,563 Columbus Ave,02118,42.3412,-71.0815,Citizens Connect App
+,101004114152,2022-01-02 16:18:30,2022-01-10 08:30:00,2022-01-02 16:32:54,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 16:32:54 EST 2022 Noted This not not a city park  ,Litter/Ground Maintenance - Wellington Green (BPRD),Parks & Recreation Department,Park Maintenance & Safety,Ground Maintenance,PARK_Maintenance_Ground Maintenance,PARK,https://311.boston.gov/media/boston/report/photos/61d2169605bbcf180c2a4d65/photo_20220102_161627.jpg,,563 Columbus Ave  Roxbury  MA  02118,4,1C,7,D4,South End,6,Ward 4,0404,563 Columbus Ave,02118,42.3412,-71.0815,Citizens Connect App
        "#.trim().to_string()
    }
}

#[test]
fn diff_diff_left_and_original_right_sort_diff_result_by_lines_by_default() {
    let wrk = Workdir::new("diff");
    let test_file = wrk.load_test_file("boston311-100-diff.csv");
    let test_file2 = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("diff");
    cmd.arg(test_file).arg(test_file2);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let diff_result_file_name = "diff_result_diff_left_original_right.csv";

    wrk.create(diff_result_file_name, got);

    let mut cmd = wrk.command("select");
    // select all columns
    cmd.arg("1-").arg(diff_result_file_name);

    let actual: String = wrk.stdout(&mut cmd);
    let expected = create_expected_diff_result_when_sorting_by_lines_diff_is_left_arg_and_original_is_right_arg();

    assert_eq!(dos2unix(&actual), dos2unix(&expected));

    fn create_expected_diff_result_when_sorting_by_lines_diff_is_left_arg_and_original_is_right_arg(
    ) -> String {
        r#"
diffresult,case_enquiry_id,open_dt,target_dt,closed_dt,ontime,case_status,closure_reason,case_title,subject,reason,type,queue,department,submittedphoto,closedphoto,location,fire_district,pwd_district,city_council_district,police_district,neighborhood,neighborhood_services_district,ward,precinct,location_street_name,location_zipcode,latitude,longitude,source
-,101004113747,2022-01-01 23:46:09,2022-01-17 08:30:00,2022-01-02 11:04:10,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 11:03:10 EST 2022 Noted Case noted. Duplicate case. Posts already marked for contractor to repair.  ,Street Light Outages,Public Works Department,Street Lights,Street Light Outages,PWDx_Street Light Outages,PWDx,https://311.boston.gov/media/boston/report/photos/61d12e0705bbcf180c29cfc2/report.jpg,,103 N Beacon St  Brighton  MA  02135,11,04,9,D14,Brighton,15,22,2205,103 N Beacon St,02135,42.3549,-71.143,Citizens Connect App
+,101004113747,2022-01-01 23:46:09,2022-01-17 08:30:00,2022-01-02 11:03:10,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 11:03:10 EST 2022 Noted Case noted. Duplicate case. Posts already marked for contractor to repair.  ,Street Light Outages,Public Works Department,Street Lights,Street Light Outages,PWDx_Street Light Outages,PWDx,https://311.boston.gov/media/boston/report/photos/61d12e0705bbcf180c29cfc2/report.jpg,,103 N Beacon St  Brighton  MA  02135,11,04,9,D14,Brighton,15,22,2205,103 N Beacon St,02135,42.3549,-71.143,Citizens Connect App
-,101004114377,2022-01-03 07:50:09,2022-01-04 08:30:00,2022-01-03 10:35:57,ONTIME,Closed,Case Closed. Closed date : 2022-01-03 10:35:57.797 Case Resolved Vehicles mere moved will check again sir ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,,,618 E Sixth St  South Boston  MA  02127,6,05,2,C6,South Boston / South Boston Waterfront,5,Ward 6,0606,618 E Sixth St,02127,42.3332,-71.0357,Citizens Connect App
+,101004114377,2022-01-03 07:50:09,2022-01-04 08:30:00,2022-01-03 10:35:57,ONTIME,Closed,Case Closed. Closed date : 2022-01-03 10:35:57.797 Case Resolved Vehicles mere moved will check again  ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,,,618 E Sixth St  South Boston  MA  02127,6,05,2,C6,South Boston / South Boston Waterfront,5,Ward 6,0606,618 E Sixth St,02127,42.3332,-71.0357,Citizens Connect App
-,101004114069,2022-01-02 14:11:49,2022-01-05 08:30:00,2022-01-03 06:52:40,ONTIME,Closed,Case Closed. Closed date : Mon Jan 03 06:52:40 EST 2022 Resolved No violation found at this time today is trash day.  ,Improper Storage of Trash (Barrels),Public Works Department,Code Enforcement,Improper Storage of Trash (Barrels),PWDx_Code Enforcement,PWDx,https://311.boston.gov/media/boston/report/photos/61d1f8e905bbcf180c2a3d7f/report.jpg,,22 Henchman St  Boston  MA  02109,3,1B,1,A1,Downtown / Financial District,3,Ward 3,0302,22 Henchman St,02109,42.3674,-71.0537,Citizens Connect App
+,101004114069,2022-01-02 14:11:49,2022-01-05 08:30:00,2022-01-03 06:52:40,ONTIME,Closed,Case Closed. Closed date : Mon Jan 03 06:52:40 EST 2022 Resolved No violation found at this time  today is trash day.  ,Improper Storage of Trash (Barrels),Public Works Department,Code Enforcement,Improper Storage of Trash (Barrels),PWDx_Code Enforcement,PWDx,https://311.boston.gov/media/boston/report/photos/61d1f8e905bbcf180c2a3d7f/report.jpg,,22 Henchman St  Boston  MA  02109,3,1B,1,A1,Downtown / Financial District,3,Ward 3,0302,22 Henchman St,02109,42.3674,-71.0537,Citizens Connect App
-,101004114152,2022-01-02 16:18:30,2022-01-10 08:30:00,2022-01-02 16:32:54,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 16:32:54 EST 2022 Noted This not not a city park  ,Litter/Ground Maintenance - Wellington Green (BPRD),Parks & Recreation Department,Park Maintenance & Safety,Ground Maintenance,PARK_Maintenance_Ground Maintenance,PARK,https://311.boston.gov/media/boston/report/photos/61d2169605bbcf180c2a4d65/photo_20220102_161627.jpg,,563 Columbus Ave  Roxbury  MA  02118,4,1C,7,D4,South End,6,Ward 4,0404,563 Columbus Ave,02118,42.3412,-71.0815,Citizens Connect App
+,101004114152,2022-01-02 16:18:30,2022-01-10 08:30:00,2022-01-02 16:32:54,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 16:32:54 EST 2022 Noted This not not a city park  ,Litter / Ground Maintenance - Wellington Green (BPRD),Parks & Recreation Department,Park Maintenance & Safety,Ground Maintenance,PARK_Maintenance_Ground Maintenance,PARK,https://311.boston.gov/media/boston/report/photos/61d2169605bbcf180c2a4d65/photo_20220102_161627.jpg,,563 Columbus Ave  Roxbury  MA  02118,4,1C,7,D4,South End,6,Ward 4,0404,563 Columbus Ave,02118,42.3412,-71.0815,Citizens Connect App
        "#.trim().to_string()
    }
}

#[test]
fn diff_sort_diff_result_by_first_column() {
    let wrk = Workdir::new("diff");
    let test_file = wrk.load_test_file("boston311-100.csv");
    let test_file2 = wrk.load_test_file("boston311-100-diff.csv");

    let mut cmd = wrk.command("diff");
    cmd.arg("--sort-columns")
        .arg("0")
        .arg(test_file)
        .arg(test_file2);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let diff_result_file_name = "diff_result_original_left_diff_right_sort_columns.csv";

    wrk.create(diff_result_file_name, got);

    let mut cmd = wrk.command("select");
    // select all columns
    cmd.arg("1-").arg(diff_result_file_name);

    let actual: String = wrk.stdout(&mut cmd);
    let expected = create_expected_diff_result_when_sorting_by_first_column();

    assert_eq!(dos2unix(&actual), dos2unix(&expected));

    fn create_expected_diff_result_when_sorting_by_first_column() -> String {
        r#"
diffresult,case_enquiry_id,open_dt,target_dt,closed_dt,ontime,case_status,closure_reason,case_title,subject,reason,type,queue,department,submittedphoto,closedphoto,location,fire_district,pwd_district,city_council_district,police_district,neighborhood,neighborhood_services_district,ward,precinct,location_street_name,location_zipcode,latitude,longitude,source
-,101004113747,2022-01-01 23:46:09,2022-01-17 08:30:00,2022-01-02 11:03:10,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 11:03:10 EST 2022 Noted Case noted. Duplicate case. Posts already marked for contractor to repair.  ,Street Light Outages,Public Works Department,Street Lights,Street Light Outages,PWDx_Street Light Outages,PWDx,https://311.boston.gov/media/boston/report/photos/61d12e0705bbcf180c29cfc2/report.jpg,,103 N Beacon St  Brighton  MA  02135,11,04,9,D14,Brighton,15,22,2205,103 N Beacon St,02135,42.3549,-71.143,Citizens Connect App
+,101004113747,2022-01-01 23:46:09,2022-01-17 08:30:00,2022-01-02 11:04:10,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 11:03:10 EST 2022 Noted Case noted. Duplicate case. Posts already marked for contractor to repair.  ,Street Light Outages,Public Works Department,Street Lights,Street Light Outages,PWDx_Street Light Outages,PWDx,https://311.boston.gov/media/boston/report/photos/61d12e0705bbcf180c29cfc2/report.jpg,,103 N Beacon St  Brighton  MA  02135,11,04,9,D14,Brighton,15,22,2205,103 N Beacon St,02135,42.3549,-71.143,Citizens Connect App
-,101004114069,2022-01-02 14:11:49,2022-01-05 08:30:00,2022-01-03 06:52:40,ONTIME,Closed,Case Closed. Closed date : Mon Jan 03 06:52:40 EST 2022 Resolved No violation found at this time  today is trash day.  ,Improper Storage of Trash (Barrels),Public Works Department,Code Enforcement,Improper Storage of Trash (Barrels),PWDx_Code Enforcement,PWDx,https://311.boston.gov/media/boston/report/photos/61d1f8e905bbcf180c2a3d7f/report.jpg,,22 Henchman St  Boston  MA  02109,3,1B,1,A1,Downtown / Financial District,3,Ward 3,0302,22 Henchman St,02109,42.3674,-71.0537,Citizens Connect App
+,101004114069,2022-01-02 14:11:49,2022-01-05 08:30:00,2022-01-03 06:52:40,ONTIME,Closed,Case Closed. Closed date : Mon Jan 03 06:52:40 EST 2022 Resolved No violation found at this time today is trash day.  ,Improper Storage of Trash (Barrels),Public Works Department,Code Enforcement,Improper Storage of Trash (Barrels),PWDx_Code Enforcement,PWDx,https://311.boston.gov/media/boston/report/photos/61d1f8e905bbcf180c2a3d7f/report.jpg,,22 Henchman St  Boston  MA  02109,3,1B,1,A1,Downtown / Financial District,3,Ward 3,0302,22 Henchman St,02109,42.3674,-71.0537,Citizens Connect App
-,101004114152,2022-01-02 16:18:30,2022-01-10 08:30:00,2022-01-02 16:32:54,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 16:32:54 EST 2022 Noted This not not a city park  ,Litter / Ground Maintenance - Wellington Green (BPRD),Parks & Recreation Department,Park Maintenance & Safety,Ground Maintenance,PARK_Maintenance_Ground Maintenance,PARK,https://311.boston.gov/media/boston/report/photos/61d2169605bbcf180c2a4d65/photo_20220102_161627.jpg,,563 Columbus Ave  Roxbury  MA  02118,4,1C,7,D4,South End,6,Ward 4,0404,563 Columbus Ave,02118,42.3412,-71.0815,Citizens Connect App
+,101004114152,2022-01-02 16:18:30,2022-01-10 08:30:00,2022-01-02 16:32:54,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 16:32:54 EST 2022 Noted This not not a city park  ,Litter/Ground Maintenance - Wellington Green (BPRD),Parks & Recreation Department,Park Maintenance & Safety,Ground Maintenance,PARK_Maintenance_Ground Maintenance,PARK,https://311.boston.gov/media/boston/report/photos/61d2169605bbcf180c2a4d65/photo_20220102_161627.jpg,,563 Columbus Ave  Roxbury  MA  02118,4,1C,7,D4,South End,6,Ward 4,0404,563 Columbus Ave,02118,42.3412,-71.0815,Citizens Connect App
-,101004114377,2022-01-03 07:50:09,2022-01-04 08:30:00,2022-01-03 10:35:57,ONTIME,Closed,Case Closed. Closed date : 2022-01-03 10:35:57.797 Case Resolved Vehicles mere moved will check again  ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,,,618 E Sixth St  South Boston  MA  02127,6,05,2,C6,South Boston / South Boston Waterfront,5,Ward 6,0606,618 E Sixth St,02127,42.3332,-71.0357,Citizens Connect App
+,101004114377,2022-01-03 07:50:09,2022-01-04 08:30:00,2022-01-03 10:35:57,ONTIME,Closed,Case Closed. Closed date : 2022-01-03 10:35:57.797 Case Resolved Vehicles mere moved will check again sir ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,,,618 E Sixth St  South Boston  MA  02127,6,05,2,C6,South Boston / South Boston Waterfront,5,Ward 6,0606,618 E Sixth St,02127,42.3332,-71.0357,Citizens Connect App
        "#.trim().to_string()
    }
}
