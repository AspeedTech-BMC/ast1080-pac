#[doc = "Register `HCIEXTCAP00C` reader"]
pub type R = crate::R<Hciextcap00cSpec>;
#[doc = "Register `HCIEXTCAP00C` writer"]
pub type W = crate::W<Hciextcap00cSpec>;
#[doc = "Field `REGHWIDI3CPRODUCTID` reader - REG_HW_ID_I3C_PRODUCT_ID"]
pub type Reghwidi3cproductidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_HW_ID_I3C_PRODUCT_ID"]
    #[inline(always)]
    pub fn reghwidi3cproductid(&self) -> Reghwidi3cproductidR {
        Reghwidi3cproductidR::new(self.bits)
    }
}
impl W {}
#[doc = "HW\\_ID\\_I3C\\_PRODUCT\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap00cSpec;
impl crate::RegisterSpec for Hciextcap00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap00c::R`](R) reader structure"]
impl crate::Readable for Hciextcap00cSpec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap00c::W`](W) writer structure"]
impl crate::Writable for Hciextcap00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP00C to value 0"]
impl crate::Resettable for Hciextcap00cSpec {}
