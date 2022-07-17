package com.sloth.app.servlets

import com.sloth.app.services.ContentManagementService
import org.mockito.Mockito._
import org.scalamock.scalatest.MockFactory
import org.scalatest.matchers.should.Matchers
import org.scalatra.test.scalatest._

class ContentManagementServletTests extends ScalatraFunSuite with MockFactory with Matchers {

  test("DELETE /content/clear on ContentManagementServlet should return status 204") {
    val cmsMocked = mock[ContentManagementService]
    addServlet(new ContentManagementServlet(cmsMocked), "/content")

    (cmsMocked.deleteAllContent _).expects()

    delete("/content/clear") {
      status should equal(204)
    }
  }

  test("DELETE /content/clear on ContentManagementServlet handles exception and should return status 500") {
    val cmsMocked = mock[ContentManagementService]
    addServlet(new ContentManagementServlet(cmsMocked), "/*")

    (cmsMocked.deleteAllContent _).expects().onCall(() => throw new Exception())

    delete("/content/clear") {
      status should equal(500)
    }
  }

  override def header = ???
}
