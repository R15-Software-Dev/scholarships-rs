# Scholarship Application Software Requirements Specification

Version 1.0

Prepared by Lucas Vas

CT Regional School District 15

Created 08/21/2025

Table of Contents
=================

* [Revision History](#revision-history)
* 1 [Introduction](#1-introduction)
    * 1.1 [Document Purpose](#11-document-purpose)
    * 1.2 [Product Scope](#12-product-scope)
    * 1.3 [Definitions, Acronyms and Abbreviations](#13-definitions-acronyms-and-abbreviations)
    * 1.4 [References](#14-references)
    * 1.5 [Document Overview](#15-document-overview)
* 2 [Product Overview](#2-product-overview)
    * 2.1 [Product Perspective](#21-product-perspective)
    * 2.2 [Product Functions](#22-product-functions)
    * 2.3 [Product Constraints](#23-product-constraints)
    * 2.4 [User Characteristics](#24-user-characteristics)
    * 2.5 [Assumptions and Dependencies](#25-assumptions-and-dependencies)
    * 2.6 [Apportioning of Requirements](#26-apportioning-of-requirements)
* 3 [Requirements](#3-requirements)
    * 3.1 [External Interfaces](#31-external-interfaces)
        * 3.1.1 [User Interfaces](#311-user-interfaces)
        * 3.1.2 [Hardware Interfaces](#312-hardware-interfaces)
        * 3.1.3 [Software Interfaces](#313-software-interfaces)
    * 3.2 [Functional](#32-functional)
    * 3.3 [Quality of Service](#33-quality-of-service)
        * 3.3.1 [Performance](#331-performance)
        * 3.3.2 [Security](#332-security)
        * 3.3.3 [Reliability](#333-reliability)
        * 3.3.4 [Availability](#334-availability)
    * 3.4 [Compliance](#34-compliance)
    * 3.5 [Design and Implementation](#35-design-and-implementation)
        * 3.5.1 [Installation](#351-installation)
        * 3.5.2 [Distribution](#352-distribution)
        * 3.5.3 [Maintainability](#353-maintainability)
        * 3.5.4 [Reusability](#354-reusability)
        * 3.5.5 [Portability](#355-portability)
        * 3.5.7 [Deadline](#357-deadline)
* 4 [Verification](#4-verification)
* 5 [Appendixes](#5-appendixes)

## Revision History

| Name      | Date    | Reason For Changes | Version |
|-----------|---------|--------------------|---------|
| Lucas Vas | 08/2025 | Initial Release    | 1.0     |

## 1. Introduction

### 1.1 Document Purpose

This document describes the functional and non-functional requirements of the Scholarship Application. It is intended to
provide a high-level set of requirements that will guide the development of the application.

### 1.2 Product Scope

This product is a web application that provides an all-in-one solution for scholarship providers and students. Providers
can create scholarships and students can apply for them.

### 1.3 Definitions, Acronyms, and Abbreviations

- `cargo`: The standard Rust compiler and package manager.
- `git`: The version control system.
- "Student": Any user that is a high school senior student.
- "Provider": Any user that is a scholarship provider.
- "Admin": Any user that has been given explicit permission to access the admin panel of this application.
- "Application": The scholarship application.

### 1.4 References

There are no external references in this document.

### 1.5 Document Overview

This document is split into the following sections:

- Product Overview: General information about the application.
- Requirements: The specific requirements of the application.
- Verification: The verification methods used to test the application.
- Appendices: Additional information about the application.

## 2. Product Overview

### 2.1 Product Perspective

The application is a web application that allows students to apply for scholarships. It is intended to replace the older
system, which is a sheet of paper that must be filled out by hand, one for each desired scholarship.

### 2.2 Product Functions

The application will have the following high-level functions:

- Students shall be able to apply for scholarships.
- Providers shall be able to manage their scholarships.
- Administrators shall be able to manage all registered users and their applications/scholarships.
- Providers shall be able to see all students that are eligible for their scholarships.

### 2.3 Product Constraints

The back-end application will be designed to be portable to any operating system that supports the Rust programming
language, as it will be designed using the Leptos full-stack web framework. It shall be hosted on a cloud server, such
as AWS EC2.

All client-side functionality must be usable on the most recent versions of all major browsers, specifically Chrome,
Firefox, and Edge.

### 2.4 User Characteristics

Users are split into three groups, each with their own access level:

- Students: Students can apply for scholarships and view their application.
- Providers: Providers can create and edit their scholarships and view all their applicants.
- Administrators: Administrators can manage all users (students and providers), and view all their related information.

### 2.5 Assumptions and Dependencies

The following are dependencies that affect the design of the application:
 - The Leptos framework will be used for the front and back-end. This will allow for a single codebase to be used for
   both the front and back-end.
 - The application will be hosted on AWS EC2, or an equivalent cloud service.

The assumptions that this is based on are that Leptos will not have a major version bump and that the framework will not
change significantly. If so, this could require significant changes to the application. Another assumption is that AWS
EC2 will not experience a major change, including the cost of the service and the availability of the service.

### 2.6 Apportioning of Requirements

#### 2.6.1 User Management
> This section details all requirements for user management.
##### 2.6.1.1 (SCH-001) Student Registration
Students shall be able to sign up with their Region 15 email address and password.
##### 2.6.1.2 (SCH-002) Student Login
Students shall be able to log in with their Region 15 email address and password.
##### 2.6.1.4 (SCH-004) Provider Registration
Providers shall be able to register using their own email address and a password.
##### 2.6.1.5 (SCH-005) Provider Login
Providers shall be able to log in using their own email address and password.
##### 2.6.1.6 (SCH-006) Provider Password Reset
Providers shall be able to reset their password using their own email address.
##### 2.6.1.7 (SCH-007) Admin Registration
Administrators shall be able to register using their Region 15 email address and password.
##### 2.6.1.8 (SCH-008) Admin Login
Administrators shall be able to log in using their Region 15 email address and password.

#### 2.6.2 Student Application
> This section details all requirements for the student application.
##### 2.6.2.1 (SCH-009) Student General Information
Students shall be able to enter their general information. This shall include their name, email address, and phone
number.
##### 2.6.2.2 (SCH-010) Student Additional Information
Students shall be able to enter additional information about themselves. This information may include their grades,
intended major, athletic/sports participation, and other relevant information.
##### 2.6.2.3 (SCH-011) Student General Applications
Students shall automatically be registered for all scholarships that they are eligible for with their general
information.
##### 2.6.2.4 (SCH-012) Student Scholarship Applications
Students shall be able to apply for specific scholarships. Students shall have the ability to select which scholarships
they would like to apply for and select the information from their profile that they would like to include in that
application.

#### 2.6.3 Provider Management
> This section details all requirements for providers to enter their scholarship's information.
##### 2.6.3.1 (SCH-013) Provider General Information
Providers shall be able to enter their own general information. This shall include their name, email address, and phone
number.
##### 2.6.3.2 (SCH-014) Multiple Scholarships
Providers shall be able to create multiple scholarships.
##### 2.6.3.2 (SCH-015) Provider Scholarship Information
Providers shall be able to enter the general information about their scholarship(s). This shall include the scholarship's
name, description, and monetary value.
##### 2.6.3.3 (SCH-016) Provider Scholarship Additional Information
Providers shall be able to request additional information from their applicants. Some examples of this information may
include the number of credits completed, the number of community service hours, athletic/sports participation, and 
financial aid information. They shall also be able to request that their applicants write an essay.

#### 2.6.4 Administrator Functions
> This section details all requirements for administrators to manage all users and their applications/scholarships.
##### 2.6.4.1 (SCH-017) Admin User Management
Administrators shall be able to manage all users. This includes the ability to create, edit, and delete users and all
their information.
##### 2.6.4.2 (SCH-018) Admin Scholarship Management
Administrators shall be able to manage all scholarships. This includes the ability to create, edit, and delete
scholarships and all their information.

## 3. Requirements

> This section specifies the software product's requirements. Specify all the software requirements to a level of
> detail sufficient to enable designers to design a software system to satisfy those requirements and to enable testers
> to test that the software system satisfies those requirements.

### 3.1 External Interfaces
#### 3.1.1 User interfaces
##### 3.1.1.1 Student Interfaces
> This section details all interfaces that are required for a student to use the application.
###### 3.1.1.1.1 General Application
This interface will be used by students to enter typical identifiers and information about themselves.
###### 3.1.1.1.2 Individual Scholarship Applications
This interface shall be used by students to apply for specific scholarships that require additional information, past
their identifying information.
##### 3.1.1.2 Provider Interfaces
> This section details all interfaces that are required for a provider to use the application.
###### 3.1.1.2.1 General Information
This interface shall be used by providers to enter their scholarship's general identifying information.
###### 3.1.1.2.2 Scholarship-Specific Information
This interface shall be used by providers to enter specific information about their scholarship. This includes the list
of requirements for the scholarship, such as a specific number of credits, an amount of community service, and other 
relevant information.
##### 3.1.1.3 Admin Interfaces
> This section details all interfaces that are required for an administrator to use the application.
###### 3.1.1.3.1 Provider Management
This interface shall be used by administrators to manage all providers. This includes the ability to create, edit, and
delete providers and all their information.
###### 3.1.1.3.2 Student Management
This interface shall be used by administrators to manage all students. This includes the ability to create, edit, and
delete students and all their information.
###### 3.1.1.3.3 Scholarship Management
This interface shall be used by administrators to manage all scholarships. This includes the ability to create, edit, and
delete scholarships and all their information.

#### 3.1.2 Hardware interfaces

There are no hardware interfaces for this project.

#### 3.1.3 Software interfaces

There are no software interfaces for this project.

### 3.2 Functional

> This section specifies the requirements of functional effects that the software-to-be is to have on its environment.

### 3.3 Quality of Service

> This section states additional, quality-related property requirements that the functional effects of the software
> should present.

#### 3.3.1 Performance

Under a typical load, the application shall be able to load and process data within 3 seconds. All other performance
requirements shall be specified in terms of this average response time.

#### 3.3.2 Security

User information shall be protected from unauthorized access. Users shall only be able to access their own information 
unless they are an administrator. However, administrators are not allowed to access specific user information, such as
their passwords or any financial information.

#### 3.3.3 Reliability

The application shall be able to recover from any expectable failure.

#### 3.3.4 Availability

The application shall be available 24/7, except during maintenance.

### 3.4 Compliance

There are no additional compliance requirements.

### 3.5 Design and Implementation

#### 3.5.1 Installation

The application shall be installable on any operating system that supports the Rust programming language, however it is
recommended that the application be tested and installed on a Linux distribution.

#### 3.5.2 Distribution

The application shall be available through the internet as a web application.

#### 3.5.3 Maintainability

All components of the application shall be modular if possible.

#### 3.5.4 Reusability

The application shall be able to be reused over the course of multiple school years.

#### 3.5.5 Portability

The application shall be able to run on any operating system that supports the Rust programming language.

#### 3.5.7 Deadline

The first release of this application shall be completed before the end of the current year (2025).

## 4. Verification

> This section provides the verification approaches and methods planned to qualify the software. The information items
> for verification are recommended to be given in a parallel manner with the requirement items in Section 3. The purpose
> of the verification process is to provide objective evidence that a system or system element fulfills its specified
> requirements and characteristics.

<!-- TODO: give more guidance, similar to section 3 -->
<!-- ieee 15288:2015 -->

## 5. Appendixes