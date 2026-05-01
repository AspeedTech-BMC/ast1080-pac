#[doc = "Register `HCIEXTCAP09C` reader"]
pub type R = crate::R<Hciextcap09cSpec>;
#[doc = "Register `HCIEXTCAP09C` writer"]
pub type W = crate::W<Hciextcap09cSpec>;
#[doc = "Field `REGDMAMBUSARBDBG3` reader - REG_DMA_MBUS_ARB_DBG_3"]
pub type Regdmambusarbdbg3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DMA_MBUS_ARB_DBG_3"]
    #[inline(always)]
    pub fn regdmambusarbdbg3(&self) -> Regdmambusarbdbg3R {
        Regdmambusarbdbg3R::new(self.bits)
    }
}
impl W {}
#[doc = "DMA\\_MBUS\\_ARB\\_DBG\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap09cSpec;
impl crate::RegisterSpec for Hciextcap09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap09c::R`](R) reader structure"]
impl crate::Readable for Hciextcap09cSpec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap09c::W`](W) writer structure"]
impl crate::Writable for Hciextcap09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP09C to value 0"]
impl crate::Resettable for Hciextcap09cSpec {}
