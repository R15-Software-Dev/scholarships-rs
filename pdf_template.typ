#set page(margin: 40pt)
#set block(breakable: false)

#let map_block(header, val) = {
  block(
    width: 100%,
    fill: rgb(200, 0, 0),
    radius: 4pt,
    below: 0pt,
    above: 4pt,
    inset: 2pt,
    [
      #show heading: set text(white)
      #show heading: set text(size: 15pt)
      #stack(dir: ttb,
        block(
          inset: 7pt,
          align(horizon)[= #header]
        ),
        align(center)[#block(
          width: 100%,
          fill: white,
          inset: 8pt,
          radius: 3pt,
          align(left)[#val]
        )],
      )
    ]
  )
}

#let info_line(subject, key, dict: student) = [

  *#subject* #dict.at(key, default: "N/A")
]

#let info_line_list(subject, key, dict: student) = [

  *#subject* #dict.at(key, default: "N/A").join(", ")
]

#show heading: set align(center)
#show heading: set text(size: 20pt)
#v(10cm)
= #student.first_name #student.last_name
#show heading: set text(size: 17pt)
== Region 15 General Scholarship Application
#pagebreak()

#show heading: set block(below: 20pt, above: 30pt)
#show heading: set text(size: 20pt)
= Student Demographic Information

#info_line("First Name:", "first_name")
#info_line("Last Name:", "last_name")
#info_line("Date of Birth:", "dob")
#info_line("Gender:", "gender")
#info_line("Phone Number:", "phone_number")
#info_line("Email:","email")
#info_line("Street Address:", "street_address")
#info_line("Town:", "town")

= Academic Information

#info_line("Unweighted GPA:", "unweighted_gpa")
#info_line("Weighted GPA:", "weighted_gpa")
#info_line("Highest SAT Score:", "sat_score")
#info_line("Highest ACT Score:", "act_score")
#info_line("Academic Honors:", "academic_honors")

= Sports Participation
#if student.at("sports_participation", default: ()).len() == 0 [
  Student did not enter any sports participation.
] else {
  for pair in student.sports_participation {
    map_block(pair.at("sport_name"), [
      #info_line("Special Achievements:", "achievements", dict: pair)
      #info_line_list("Grades Participated:", "grades", dict: pair)
    ])
  }
}

= Extracurricular Activities
#if student.at("extracurricular", default: ()).len() == 0 [
  Student did not enter any extracurricular activities.
] else {
  for pair in student.extracurricular {
    map_block(pair.at("activity_name"), [
      #info_line("Number of hours per week:", "num_hours", dict: pair)
      #info_line("Number of weeks participated:", "num_weeks", dict: pair)
      #info_line("Special Involvement:", "special_involvement", dict: pair)
    ])
  }
}

= Work Experience
#if student.at("work_experience", default: ()).len() == 0 [
  Student did not enter any work experience.
] else {
  for pair in student.work_experience {
    map_block(pair.at("employer"), [
      #info_line("Job Title:", "job_title", dict: pair)
      #info_line("Start Date:", "start_date", dict: pair)
      #info_line("End Date:", "end_date", dict: pair)
      #info_line("Number of hours per week:", "num_hours", dict: pair)
    ])
  }
}

= University Information

#info_line("University Name:", "college_name")
#info_line("University Street Address:", "college_city")
#info_line("University State:", "college_state")
#info_line("University ZIP:", "college_zip")
#info_line("Has the student received an acceptance:", "college_acceptance")
#info_line("Student's chosen major:", "major")
#info_line("Student's intended career:", "intended_career")

= Family Information

#info_line("Total number of children in student's family:", "num_children")
#info_line("Total number of children currently attending college:", "num_children_college")
#info_line("Parent/Guardian 1 Name:", "parent_one_name")
#info_line("Parent/Guardian 1 Relationship:", "parent_one_relationship")
#info_line("Parent/Guardian 1 Occupation:", "parent_one_occupation")
#info_line("Parent/Guardian 1 Employer:", "parent_one_employer")
#info_line("Parent/Guardian 2 Name:", "parent_two_name")
#info_line("Parent/Guardian 2 Relationship:", "parent_two_relationship")
#info_line("Parent/Guardian 2 Occupation:", "parent_two_occupation")
#info_line("Parent/Guardian 2 Employer:", "parent_two_employer")

= Extra Eligibility Information

#info_line("Did the student attend BAS:", "attend_bas")
#info_line("Is the student a member of Midd-South Catholic Church:", "middsouth_church")
#info_line("Does the student have a family member who has served in the US Military:", "family_military_service")
#info_line("Has the student participated in Pomperaug Youth Baseball:", "youth_baseball")
#info_line("Has the student participated in the Panthers Aquatic Club:", "aquatic_club")
#info_line("Does the student have a family member in the Region 15 PEA:", "pea_member")
#info_line("Has the student participated in the PHS Music Program:", "music_program")
