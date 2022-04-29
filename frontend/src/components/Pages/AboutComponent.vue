<template>
    <!-- ======= About Us Section ======= -->
    <section id="about" class="about">
      <div class="container">

        <div class="section-title">
          <h2>About Us</h2>
        </div>

        <div class="row">
          <div class="col-lg-6 order-1 order-lg-2">
            <img :src="require('@/assets/images/meeting.jpg')" class="img-fluid" alt="">
          </div>
          <div class="col-lg-6 pt-4 pt-lg-0 order-2 order-lg-1 content">
            <h3>We created the MeetingBy project in order to help people to meet with their idols, gain useful knowledge, new experience, all money will be donated to charity.</h3>
            <hr>
            <p>
              Also you can help animals, children which need your help, people with disabilities, our planet with environment problems, etc.
            </p>
          </div>
        </div>

        <div id="why-us" class="why-us">
          <div  class="row no-gutters">

            <div class="col-lg-4 col-md-6 content-item">
              <span><font-awesome-icon icon="house-medical" /></span>
              <h4>Medicine</h4>
              <p>You can save the lives of people who need medical help but don't have the money to get it.</p>
            </div>

            <div class="col-lg-4 col-md-6 content-item">
              <span><font-awesome-icon icon="children" /></span>
              <h4>Children</h4>
              <p>You can help children become their dreams come true. Educational program, medical program and other programs for children.
              </p>
            </div>

            <div class="col-lg-4 col-md-6 content-item">
              <span><font-awesome-icon icon="wheelchair" /></span>
              <h4>Disability</h4>
              <p>Help people with disabilities, you can make their lives better.
              </p>
            </div>

            <div class="col-lg-4 col-md-6 content-item">
              <span><font-awesome-icon icon="seedling" /></span>
              <h4>Environment</h4>
              <p>You can help our planet, which is suffering from environmental problems.  We work with  organizations that are involved in saving our planet.
              </p>
            </div>

            <div class="col-lg-4 col-md-6 content-item">
              <span><font-awesome-icon icon="paw" /></span>
              <h4>Animal</h4>
              <p>Animals in Ukrainian zoos are starving, you can save them.</p>
            </div>

            <div class="col-lg-4 col-md-6 content-item">
              <span><font-awesome-icon icon="graduation-cap" /></span>
              <h4>Education</h4>
              <p>Make your contribution to the development of science </p>
            </div>

          </div>
        </div>
      </div>
    </section><!-- End About Us Section -->
</template>

<script>
export default {
  name: "AboutComponent"
}
</script>

<style scoped>

/*--------------------------------------------------------------
# About Us
--------------------------------------------------------------*/
.about .content h3 {
  font-weight: 700;
  font-size: 28px;
}

.about .content ul {
  list-style: none;
  padding: 0;
}

.about .content ul li {
  padding-bottom: 10px;
}

.about .content ul i {
  font-size: 20px;
  padding-right: 4px;
  color: #5cb874;
}

.about .content p:last-child {
  margin-bottom: 0;
}


/*--------------------------------------------------------------
# Why Us
--------------------------------------------------------------*/
.why-us {
  padding: 40px 0;
}

.why-us .row {
  overflow: hidden;
}

.why-us .content-item {
  padding: 40px;
  border-left: 1px solid #eee;
  border-bottom: 1px solid #eee;
  margin: -1px;
}

.why-us .content-item span {
  display: block;
  font-size: 24px;
  font-weight: 400;
  color: #6ec083;
}

.why-us .content-item h4 {
  font-size: 26px;
  font-weight: 300;
  padding: 0;
  margin: 20px 0;
}

.why-us .content-item p {
  color: #aaaaaa;
  font-size: 15px;
  margin: 0;
  padding: 0;
}

@media (max-width: 768px) {
  .why-us .content-item {
    padding: 40px 0;
  }
}


</style>