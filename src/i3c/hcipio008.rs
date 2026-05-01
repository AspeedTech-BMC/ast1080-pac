#[doc = "Register `HCIPIO008` reader"]
pub type R = crate::R<Hcipio008Spec>;
#[doc = "Register `HCIPIO008` writer"]
pub type W = crate::W<Hcipio008Spec>;
#[doc = "Field `REGDATAPORT` reader - REG_DATA_PORT"]
pub type RegdataportR = crate::FieldReader<u32>;
#[doc = "Field `REGDATAPORT` writer - REG_DATA_PORT"]
pub type RegdataportW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DATA_PORT"]
    #[inline(always)]
    pub fn regdataport(&self) -> RegdataportR {
        RegdataportR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_DATA_PORT"]
    #[inline(always)]
    pub fn regdataport(&mut self) -> RegdataportW<Hcipio008Spec> {
        RegdataportW::new(self, 0)
    }
}
#[doc = "XFER\\_DATA\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio008Spec;
impl crate::RegisterSpec for Hcipio008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio008::R`](R) reader structure"]
impl crate::Readable for Hcipio008Spec {}
#[doc = "`write(|w| ..)` method takes [`hcipio008::W`](W) writer structure"]
impl crate::Writable for Hcipio008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO008 to value 0"]
impl crate::Resettable for Hcipio008Spec {}
