#[doc = "Register `I3CCONTROL090` reader"]
pub type R = crate::R<I3ccontrol090Spec>;
#[doc = "Register `I3CCONTROL090` writer"]
pub type W = crate::W<I3ccontrol090Spec>;
#[doc = "Field `REGRINGRDMASWCLR` reader - REG_RING_RDMA_SW_CLR"]
pub type RegringrdmaswclrR = crate::BitReader;
#[doc = "Field `REGRINGRDMASWCLR` writer - REG_RING_RDMA_SW_CLR"]
pub type RegringrdmaswclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRINGRDMACNTTH` reader - REG_RING_RDMA_CNT_TH"]
pub type RegringrdmacntthR = crate::FieldReader;
#[doc = "Field `REGRINGRDMACNTTH` writer - REG_RING_RDMA_CNT_TH"]
pub type RegringrdmacntthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRINGRDMALENTH` reader - REG_RING_RDMA_LEN_TH"]
pub type RegringrdmalenthR = crate::FieldReader;
#[doc = "Field `REGRINGRDMALENTH` writer - REG_RING_RDMA_LEN_TH"]
pub type RegringrdmalenthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - REG_RING_RDMA_SW_CLR"]
    #[inline(always)]
    pub fn regringrdmaswclr(&self) -> RegringrdmaswclrR {
        RegringrdmaswclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - REG_RING_RDMA_CNT_TH"]
    #[inline(always)]
    pub fn regringrdmacntth(&self) -> RegringrdmacntthR {
        RegringrdmacntthR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_RING_RDMA_LEN_TH"]
    #[inline(always)]
    pub fn regringrdmalenth(&self) -> RegringrdmalenthR {
        RegringrdmalenthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - REG_RING_RDMA_SW_CLR"]
    #[inline(always)]
    pub fn regringrdmaswclr(&mut self) -> RegringrdmaswclrW<I3ccontrol090Spec> {
        RegringrdmaswclrW::new(self, 0)
    }
    #[doc = "Bits 16:23 - REG_RING_RDMA_CNT_TH"]
    #[inline(always)]
    pub fn regringrdmacntth(&mut self) -> RegringrdmacntthW<I3ccontrol090Spec> {
        RegringrdmacntthW::new(self, 16)
    }
    #[doc = "Bits 24:31 - REG_RING_RDMA_LEN_TH"]
    #[inline(always)]
    pub fn regringrdmalenth(&mut self) -> RegringrdmalenthW<I3ccontrol090Spec> {
        RegringrdmalenthW::new(self, 24)
    }
}
#[doc = "I3C\\_RDMA\\_CTL\\_090\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol090Spec;
impl crate::RegisterSpec for I3ccontrol090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol090::R`](R) reader structure"]
impl crate::Readable for I3ccontrol090Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol090::W`](W) writer structure"]
impl crate::Writable for I3ccontrol090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL090 to value 0x0380_0000"]
impl crate::Resettable for I3ccontrol090Spec {
    const RESET_VALUE: u32 = 0x0380_0000;
}
