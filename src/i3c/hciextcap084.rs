#[doc = "Register `HCIEXTCAP084` reader"]
pub type R = crate::R<Hciextcap084Spec>;
#[doc = "Register `HCIEXTCAP084` writer"]
pub type W = crate::W<Hciextcap084Spec>;
#[doc = "Field `REGDMAMBUSARBCLR` reader - REG_DMA_MBUS_ARB_CLR"]
pub type RegdmambusarbclrR = crate::BitReader;
#[doc = "Field `REGDMAMBUSARBCLR` writer - REG_DMA_MBUS_ARB_CLR"]
pub type RegdmambusarbclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_DMA_MBUS_ARB_CLR"]
    #[inline(always)]
    pub fn regdmambusarbclr(&self) -> RegdmambusarbclrR {
        RegdmambusarbclrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_DMA_MBUS_ARB_CLR"]
    #[inline(always)]
    pub fn regdmambusarbclr(&mut self) -> RegdmambusarbclrW<Hciextcap084Spec> {
        RegdmambusarbclrW::new(self, 0)
    }
}
#[doc = "DMA\\_MBUS\\_ARB\\_CLR\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap084Spec;
impl crate::RegisterSpec for Hciextcap084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap084::R`](R) reader structure"]
impl crate::Readable for Hciextcap084Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap084::W`](W) writer structure"]
impl crate::Writable for Hciextcap084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP084 to value 0"]
impl crate::Resettable for Hciextcap084Spec {}
