#[doc = "Register `HCIEXTCAP094` reader"]
pub type R = crate::R<Hciextcap094Spec>;
#[doc = "Register `HCIEXTCAP094` writer"]
pub type W = crate::W<Hciextcap094Spec>;
#[doc = "Field `REGDMAMBUSARBDBG1` reader - REG_DMA_MBUS_ARB_DBG_1"]
pub type Regdmambusarbdbg1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DMA_MBUS_ARB_DBG_1"]
    #[inline(always)]
    pub fn regdmambusarbdbg1(&self) -> Regdmambusarbdbg1R {
        Regdmambusarbdbg1R::new(self.bits)
    }
}
impl W {}
#[doc = "DMA\\_MBUS\\_ARB\\_DBG\\_1\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap094Spec;
impl crate::RegisterSpec for Hciextcap094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap094::R`](R) reader structure"]
impl crate::Readable for Hciextcap094Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap094::W`](W) writer structure"]
impl crate::Writable for Hciextcap094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP094 to value 0"]
impl crate::Resettable for Hciextcap094Spec {}
