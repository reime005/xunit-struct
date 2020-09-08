#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate log;
extern crate simple_logger;

use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, PartialEq)]

enum Root {
    #[serde(rename = "testsuites")]
    Testsuites {
        disabled: Option<String>,
        errors: Option<String>,
        failures: Option<String>,
        name: Option<String>,
        tests: Option<String>,
        time: Option<String>,
        testsuite: Vec<TestSuite>,
    },
    #[serde(rename = "testsuite")]
    Testsuite {
        name: String,
        tests: String,
        disabled: Option<String>,
        errors: Option<String>,
        failures: Option<String>,
        hostname: Option<String>,
        id: Option<String>,
        package: Option<String>,
        skipped: Option<String>,
        time: Option<String>,
        timestamp: Option<String>,
        properties: Option<Properties>,
        testcase: Vec<TestCase>,
        #[serde(rename = "system-out")]
        system_out: Option<SystemOut>,
        #[serde(rename = "system-err>")]
        system_err: Option<SystemErr>,
    },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "testsuite")]
struct TestSuite {
    name: String,
    tests: u16,
    disabled: Option<String>,
    errors: Option<String>,
    failures: Option<String>,
    hostname: Option<String>,
    id: Option<String>,
    package: Option<String>,
    skipped: Option<String>,
    time: Option<f32>,
    timestamp: Option<String>,
    properties: Option<Properties>,
    testcase: Option<Vec<TestCase>>,
    #[serde(rename = "system-out")]
    system_out: Option<SystemOut>,
    #[serde(rename = "system-err>")]
    system_err: Option<SystemErr>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "testcase")]
struct TestCase {
    assertions: Option<String>,
    classname: String,
    status: Option<String>,
    time: f32,
    skipped: Option<Skipped>,
    error: Option<Error>,
    failure: Option<Failure>,
    #[serde(rename = "system-out", default)]
    system_out: Option<SystemOut>,
    #[serde(rename = "system-err", default)]
    system_err: Option<SystemErr>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "skipped")]
struct Skipped {
    message: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "error")]
struct Error {
    message: String,
    #[serde(rename = "type", default)]
    error_type: String,
    #[serde(rename = "$value")]
    description: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "failure")]
struct Failure {
    message: String,
    #[serde(rename = "type", default)]
    failure_type: String,
    #[serde(rename = "$value")]
    description: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "system-out")]
struct SystemOut {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "system-err")]
struct SystemErr {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "property")]
struct Property {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Properties {
    #[serde(rename = "$value")]
    value: Vec<Property>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failure() {
        let junit_str = r#"<failure message="" type=""
	       >failure description</failure>"#;
        let item: Failure = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn testcase() {
        let junit_str = r#"<testcase name="name"
	      assertions="true = 1"
	      classname="testclass"
	      status="status"
	      time="0.1"
          >
           <error message="The error message. e.g., if a java exception is thrown, the return value of getMessage()"
	     type="The type of error that occured. e.g., if a java execption is thrown the full class name of the exception."
	     >error description</error>
          <failure message="The message specified in the assert."
	       type="The type of the assert."
	       >failure description</failure>
          <skipped message="message/description string why the test case was skipped. optional"/>
          <system-out>STDOUT text</system-out>
           <system-err>STDERR text</system-err>
          </testcase>
          "#;
        let item: TestCase = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn properties() {
        let junit_str = r#"<properties>
            <property name="name" value="value"/>
        </properties>"#;
        let item: Properties = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn testsuites() {
        let junit_str = r#"<?xml version="1.0" encoding="UTF-8"?>
        <testsuites name="with testsuites" >
            <testsuite name="name" tests="3">
                <testcase assertions=""classname="" status="" time="1">
                </testcase>
            </testsuite>
        </testsuites>"#;
        let item: Root = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn testsuite() {
        let junit_str = r#"<?xml version="1.0" encoding="utf-8"?>
<testsuites>
  <testsuite errors="0" failures="0" hostname="e15oms"
  name="pytest" skipped="0" tests="2" time="2.367"
  timestamp="2020-08-28T16:45:10.318141">
    <testcase classname="tests.test_fm_client_owen.Testowen"
    name="test_fm_image" time="0.750"></testcase>
    <testcase classname="tests.test_fm_client_owen.Testowen"
    name="test_fm_video" time="0.756"></testcase>
  </testsuite>
</testsuites>
"#;
        let item: Root = from_str(junit_str).unwrap();
        println!("{:#?}", item);
    }

    #[test]
    fn full() {
        let junit_str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!-- a description of the JUnit XML format and how Jenkins parses it. See also junit.xsd -->

<!-- if only a single testsuite element is present, the testsuites
     element can be omitted. All attributes are optional. -->
<testsuites disabled="" <!-- total number of disabled tests from all testsuites. -->
            errors=""   <!-- total number of tests with error result from all testsuites. -->
            failures="" <!-- total number of failed tests from all testsuites. -->
            name=""
            tests=""    <!-- total number of tests from all testsuites. Some software may expect to only see the number of successful tests from all testsuites though. -->
            time=""     <!-- time in seconds to execute all test suites. -->
	    >

  <!-- testsuite can appear multiple times, if contained in a testsuites element.
       It can also be the root element. -->
  <testsuite name=""      <!-- Full (class) name of the test for non-aggregated testsuite documents.
                               Class name without the package for aggregated testsuites documents. Required -->
	     tests=""     <!-- The total number of tests in the suite, required. -->
	     disabled=""  <!-- the total number of disabled tests in the suite. optional -->
             errors=""    <!-- The total number of tests in the suite that errored. An errored test is one that had an unanticipated problem,
                               for example an unchecked throwable; or a problem with the implementation of the test. optional -->
             failures=""  <!-- The total number of tests in the suite that failed. A failure is a test which the code has explicitly failed
                               by using the mechanisms for that purpose. e.g., via an assertEquals. optional -->
             hostname=""  <!-- Host on which the tests were executed. 'localhost' should be used if the hostname cannot be determined. optional -->
	     id=""        <!-- Starts at 0 for the first testsuite and is incremented by 1 for each following testsuite -->
	     package=""   <!-- Derived from testsuite/@name in the non-aggregated documents. optional -->
	     skipped=""   <!-- The total number of skipped tests. optional -->
	     time=""      <!-- Time taken (in seconds) to execute the tests in the suite. optional -->
	     timestamp="" <!-- when the test was executed in ISO 8601 format (2014-01-21T16:17:18). Timezone may not be specified. optional -->
	     >

    <!-- Properties (e.g., environment settings) set during test execution.
         The properties element can appear 0 or once. -->
    <properties>
      <!-- property can appear multiple times. The name and value attributres are required. -->
      <property name="" value=""/>
    </properties>

    <!-- testcase can appear multiple times, see /testsuites/testsuite@tests -->
    <testcase name=""       <!-- Name of the test method, required. -->
	      assertions="" <!-- number of assertions in the test case. optional -->
	      classname=""  <!-- Full class name for the class the test method is in. required -->
	      status=""
	      time=""       <!-- Time taken (in seconds) to execute the test. optional -->
	      >

      <!-- If the test was not executed or failed, you can specify one of the skipped, error or failure elements. -->

      <!-- skipped can appear 0 or once. optional -->
      <skipped message=""   <!-- message/description string why the test case was skipped. optional -->
	  />

      <!-- error indicates that the test errored.
           An errored test had an unanticipated problem.
           For example an unchecked throwable (exception), crash or a problem with the implementation of the test.
           Contains as a text node relevant data for the error, for example a stack trace. optional -->
      <error message="" <!-- The error message. e.g., if a java exception is thrown, the return value of getMessage() -->
	     type=""    <!-- The type of error that occured. e.g., if a java execption is thrown the full class name of the exception. -->
	     >error description</error>

      <!-- failure indicates that the test failed.
           A failure is a condition which the code has explicitly failed by using the mechanisms for that purpose.
           For example via an assertEquals.
           Contains as a text node relevant data for the failure, e.g., a stack trace. optional -->
      <failure message="" <!-- The message specified in the assert. -->
	       type=""    <!-- The type of the assert. -->
	       >failure description</failure>

      <!-- Data that was written to standard out while the test was executed. optional -->
      <system-out>STDOUT text</system-out>

      <!-- Data that was written to standard error while the test was executed. optional -->
      <system-err>STDERR text</system-err>
    </testcase>

    <!-- Data that was written to standard out while the test suite was executed. optional -->
    <system-out>STDOUT text</system-out>
    <!-- Data that was written to standard error while the test suite was executed. optional -->
    <system-err>STDERR text</system-err>
  </testsuite>
</testsuites>
"#;
        let item: Item = from_str(junit_str).unwrap();
        println!("{:#?}", item);
        assert_eq!(2 + 2, 6);
    }
}
