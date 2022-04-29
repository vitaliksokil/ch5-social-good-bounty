<template>
  <form>
    <div class="mb-3">
      <label for="nft_media" class="form-label">NFT Media</label>
      <input type="file" class="form-control" id="nft_media" @change="fileOnChange" required>
      <div class="form-text text-danger" v-if="form.errors().has('nft_media')" v-text="form.errors().get('nft_media')"></div>
      <img class="my-4" width="350" :src="form.nft_media"/>
    </div>
    <div class="mb-3">
      <label for="owner_name" class="form-label">Your name</label>
      <input type="text" class="form-control" id="owner_name" v-model='form.owner_name'>
      <div class="form-text text-danger" v-if="form.errors().has('owner_name')" v-text="form.errors().get('owner_name')"></div>
    </div>
    <div class="mb-3">
      <label for="title" class="form-label">Title</label>
      <input type="text" class="form-control" id="title" v-model='form.title'>
      <div class="form-text text-danger" v-if="form.errors().has('title')" v-text="form.errors().get('title')"></div>
    </div>
    <div class="mb-3">
      <label for="description" class="form-label">Description</label>
      <input type="text" class="form-control" id="description" v-model='form.description'>
      <div class="form-text text-danger" v-if="form.errors().has('description')" v-text="form.errors().get('description')"></div>

    </div>
    <div class="mb-3">
      <label for="ticket_price" class="form-label">Ticket price</label>
      <input type="text" class="form-control" id="ticket_price" placeholder="Ⓝ" v-model='form.ticket_price'>
      <div class="form-text text-danger" v-if="form.errors().has('ticket_price')" v-text="form.errors().get('ticket_price')"></div>

    </div>
    <div class="mb-3">
      <label for="total_tickets" class="form-label">Total tickets</label>
      <input type="text" class="form-control" id="total_tickets" v-model='form.total_tickets'>
      <div class="form-text text-danger" v-if="form.errors().has('total_tickets')" v-text="form.errors().get('total_tickets')"></div>

    </div>
    <div class="mb-3">
      <label for="place" class="form-label">Where your meeting will take place</label>
      <input type="text" class="form-control" id="place" v-model='form.place'>
      <div class="form-text text-danger" v-if="form.errors().has('place')" v-text="form.errors().get('place')"></div>

    </div>
    <div class="mb-3">
      <label for="donation_type" class="form-label">Choose to which good cause collected money will be transferred</label>
      <select id="donation_type" class="form-select" v-model='form.donation_type'>
        <option disabled value="">Select donation type</option>
        <option v-for="(item, key) in donationTypes" :key="key"  :value="item.title">
          {{ item.title }} </option>
      </select>
      <div class="form-text text-danger" v-if="form.errors().has('donation_type')" v-text="form.errors().get('donation_type')"></div>
    </div>

    <button type="submit" class="btn btn-primary" :disabled='form.empty()' @click.prevent='submit'>Submit</button>
  </form>
</template>

<script>
import form from 'vuejs-form'
import Big from "big.js";
import { NFTStorage } from 'nft.storage'

const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();

export default {
  name: "CreateMeeting",
  data(){
    return {
      form: form({
        owner_name: '',
        title: '',
        description: '',
        ticket_price: '',
        total_tickets: '',
        place: '',
        donation_type: '',
        nft_media: '',
      }).rules({
            owner_name: 'min:3|required|max:1000|string',
            title: 'min:3|required|max:1000|string',
            description: 'required|min:5|max:2000|string',
            ticket_price: 'required|numeric',
            total_tickets: 'required|integer',
            place: 'min:3|required|max:1000|string',
            donation_type: 'required|string',
            nft_media: 'required|string',
          })
    }
  },
  methods:{
    async fileOnChange(e){
      let loader = this.$loading.show();
      /* upload image to IPFS */
      const file = e.target.files[0];
      const client = new NFTStorage({ token: process.env.VUE_APP_NFT_STORAGE_TOKEN });
      const metadataCid = await client.storeBlob(file)
      const metadataUrl = "https://ipfs.io/ipfs/" + metadataCid;
      this.form.nft_media = metadataUrl;
      loader.hide();
    },

    cleanUpForm(){
      this.form.owner_name = '';
      this.form.title = '';
      this.form.description = '';
      this.form.ticket_price = '';
      this.form.total_tickets = '';
      this.form.place = '';
      this.form.donation_type = '';
      this.form.nft_media = '';
    },
    submit() {
      this.form.validate()
      return this.form.errors().any() ? this.failed() : this.passed();
    },
    failed() {
      console.log('errors: ', this.form.errors().all());
    },

    async createMeeting(
        owner_name,
        title,
        description,
        ticket_price,
        total_tickets,
        place,
        donation_type,
        nft_media,
    ) {
      await window.walletSelector.signAndSendTransaction({
        signerId: window.nearAccount.accountId,
        // receiverId: nearConfig.contractName,
        actions:[
          {
            type: "FunctionCall",
            params: {
              methodName: "add_new_meeting",
              args: {
                "owner_name": owner_name,
                "title": title,
                "description": description,
                "ticket_price": Big(ticket_price).times(10 ** 24).toFixed(),
                "total_tickets": +total_tickets,
                "place": place,
                "donation_type": donation_type,
                "nft_media": nft_media,
              },
              gas: BOATLOAD_OF_GAS,
              deposit: 0
            },
          },
        ]
      }).catch((err) => {
        console.log("Failed");
        throw err;
      });
    },
    async passed() {
      let loader = this.$loading.show();
      try {
        await this.createMeeting(
            this.form.owner_name,
            this.form.title,
            this.form.description,
            this.form.ticket_price,
            this.form.total_tickets,
            this.form.place,
            this.form.donation_type,
            this.form.nft_media,
        );
        this.$swal.fire({
          icon: 'success',
          title: 'Success',
          text: 'Your Meeting has been created!',
          footer: ``,
        })
        this.cleanUpForm();
      }catch (error){
        console.log(error.message);
        this.$swal.fire({
          icon: 'error',
          title: 'Error',
          text: error.message,
        })

      }
      loader.hide();
    }
  },
  computed: {
    donationTypes(){
      return this.$DONATION_TYPES
    }
  },
}
</script>

<style scoped>

</style>