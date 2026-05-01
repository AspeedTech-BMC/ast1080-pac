#[doc = "Register `HCIEXTCAP080` reader"]
pub type R = crate::R<Hciextcap080Spec>;
#[doc = "Register `HCIEXTCAP080` writer"]
pub type W = crate::W<Hciextcap080Spec>;
#[doc = "Field `REGDMAMBUSARBCTRL` reader - REG_DMA_MBUS_ARB_CTRL"]
pub type RegdmambusarbctrlR = crate::FieldReader<u32>;
#[doc = "Field `REGDMAMBUSARBCTRL` writer - REG_DMA_MBUS_ARB_CTRL"]
pub type RegdmambusarbctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_DMA_MBUS_ARB_CTRL"]
    #[inline(always)]
    pub fn regdmambusarbctrl(&self) -> RegdmambusarbctrlR {
        RegdmambusarbctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_DMA_MBUS_ARB_CTRL"]
    #[inline(always)]
    pub fn regdmambusarbctrl(&mut self) -> RegdmambusarbctrlW<Hciextcap080Spec> {
        RegdmambusarbctrlW::new(self, 0)
    }
}
#[doc = "DMA\\_MBUS\\_ARB\\_CTRL\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap080Spec;
impl crate::RegisterSpec for Hciextcap080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap080::R`](R) reader structure"]
impl crate::Readable for Hciextcap080Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap080::W`](W) writer structure"]
impl crate::Writable for Hciextcap080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP080 to value 0"]
impl crate::Resettable for Hciextcap080Spec {}
