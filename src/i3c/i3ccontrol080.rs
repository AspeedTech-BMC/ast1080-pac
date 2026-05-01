#[doc = "Register `I3CCONTROL080` reader"]
pub type R = crate::R<I3ccontrol080Spec>;
#[doc = "Register `I3CCONTROL080` writer"]
pub type W = crate::W<I3ccontrol080Spec>;
#[doc = "Field `REGRINGWDMASWCLR` reader - REG_RING_WDMA_SW_CLR"]
pub type RegringwdmaswclrR = crate::BitReader;
#[doc = "Field `REGRINGWDMASWCLR` writer - REG_RING_WDMA_SW_CLR"]
pub type RegringwdmaswclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRINGWDMACNTTH` reader - REG_RING_WDMA_CNT_TH"]
pub type RegringwdmacntthR = crate::FieldReader;
#[doc = "Field `REGRINGWDMACNTTH` writer - REG_RING_WDMA_CNT_TH"]
pub type RegringwdmacntthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRINGWDMALENTH` reader - REG_RING_WDMA_LEN_TH"]
pub type RegringwdmalenthR = crate::FieldReader;
#[doc = "Field `REGRINGWDMALENTH` writer - REG_RING_WDMA_LEN_TH"]
pub type RegringwdmalenthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - REG_RING_WDMA_SW_CLR"]
    #[inline(always)]
    pub fn regringwdmaswclr(&self) -> RegringwdmaswclrR {
        RegringwdmaswclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - REG_RING_WDMA_CNT_TH"]
    #[inline(always)]
    pub fn regringwdmacntth(&self) -> RegringwdmacntthR {
        RegringwdmacntthR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_RING_WDMA_LEN_TH"]
    #[inline(always)]
    pub fn regringwdmalenth(&self) -> RegringwdmalenthR {
        RegringwdmalenthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - REG_RING_WDMA_SW_CLR"]
    #[inline(always)]
    pub fn regringwdmaswclr(&mut self) -> RegringwdmaswclrW<I3ccontrol080Spec> {
        RegringwdmaswclrW::new(self, 0)
    }
    #[doc = "Bits 16:23 - REG_RING_WDMA_CNT_TH"]
    #[inline(always)]
    pub fn regringwdmacntth(&mut self) -> RegringwdmacntthW<I3ccontrol080Spec> {
        RegringwdmacntthW::new(self, 16)
    }
    #[doc = "Bits 24:31 - REG_RING_WDMA_LEN_TH"]
    #[inline(always)]
    pub fn regringwdmalenth(&mut self) -> RegringwdmalenthW<I3ccontrol080Spec> {
        RegringwdmalenthW::new(self, 24)
    }
}
#[doc = "I3C\\_WDMA\\_CTL\\_080\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol080Spec;
impl crate::RegisterSpec for I3ccontrol080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol080::R`](R) reader structure"]
impl crate::Readable for I3ccontrol080Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol080::W`](W) writer structure"]
impl crate::Writable for I3ccontrol080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL080 to value 0x0380_0000"]
impl crate::Resettable for I3ccontrol080Spec {
    const RESET_VALUE: u32 = 0x0380_0000;
}
