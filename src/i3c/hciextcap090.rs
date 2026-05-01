#[doc = "Register `HCIEXTCAP090` reader"]
pub type R = crate::R<Hciextcap090Spec>;
#[doc = "Register `HCIEXTCAP090` writer"]
pub type W = crate::W<Hciextcap090Spec>;
#[doc = "Field `REGDMAMBUSARBDBG0` reader - REG_DMA_MBUS_ARB_DBG_0"]
pub type Regdmambusarbdbg0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DMA_MBUS_ARB_DBG_0"]
    #[inline(always)]
    pub fn regdmambusarbdbg0(&self) -> Regdmambusarbdbg0R {
        Regdmambusarbdbg0R::new(self.bits)
    }
}
impl W {}
#[doc = "DMA\\_MBUS\\_ARB\\_DBG\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap090Spec;
impl crate::RegisterSpec for Hciextcap090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap090::R`](R) reader structure"]
impl crate::Readable for Hciextcap090Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap090::W`](W) writer structure"]
impl crate::Writable for Hciextcap090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP090 to value 0"]
impl crate::Resettable for Hciextcap090Spec {}
