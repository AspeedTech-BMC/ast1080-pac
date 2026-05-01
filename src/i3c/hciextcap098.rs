#[doc = "Register `HCIEXTCAP098` reader"]
pub type R = crate::R<Hciextcap098Spec>;
#[doc = "Register `HCIEXTCAP098` writer"]
pub type W = crate::W<Hciextcap098Spec>;
#[doc = "Field `REGDMAMBUSARBDBG2` reader - REG_DMA_MBUS_ARB_DBG_2"]
pub type Regdmambusarbdbg2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DMA_MBUS_ARB_DBG_2"]
    #[inline(always)]
    pub fn regdmambusarbdbg2(&self) -> Regdmambusarbdbg2R {
        Regdmambusarbdbg2R::new(self.bits)
    }
}
impl W {}
#[doc = "DMA\\_MBUS\\_ARB\\_DBG\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap098Spec;
impl crate::RegisterSpec for Hciextcap098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap098::R`](R) reader structure"]
impl crate::Readable for Hciextcap098Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap098::W`](W) writer structure"]
impl crate::Writable for Hciextcap098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP098 to value 0"]
impl crate::Resettable for Hciextcap098Spec {}
