#[doc = "Register `HCIRHS030` reader"]
pub type R = crate::R<Hcirhs030Spec>;
#[doc = "Register `HCIRHS030` writer"]
pub type W = crate::W<Hcirhs030Spec>;
#[doc = "Field `REGRINGSIZE` reader - REG_RING_SIZE"]
pub type RegringsizeR = crate::FieldReader;
#[doc = "Field `REGRINGSIZE` writer - REG_RING_SIZE"]
pub type RegringsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRESPSTRUCTSIZE` reader - REG_RESP_STRUCT_SIZE"]
pub type RegrespstructsizeR = crate::FieldReader;
#[doc = "Field `REGXFERSTRUCTSIZE` reader - REG_XFER_STRUCT_SIZE"]
pub type RegxferstructsizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - REG_RING_SIZE"]
    #[inline(always)]
    pub fn regringsize(&self) -> RegringsizeR {
        RegringsizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_RESP_STRUCT_SIZE"]
    #[inline(always)]
    pub fn regrespstructsize(&self) -> RegrespstructsizeR {
        RegrespstructsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_XFER_STRUCT_SIZE"]
    #[inline(always)]
    pub fn regxferstructsize(&self) -> RegxferstructsizeR {
        RegxferstructsizeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_RING_SIZE"]
    #[inline(always)]
    pub fn regringsize(&mut self) -> RegringsizeW<Hcirhs030Spec> {
        RegringsizeW::new(self, 0)
    }
}
#[doc = "CR\\_SETUP\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs030Spec;
impl crate::RegisterSpec for Hcirhs030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs030::R`](R) reader structure"]
impl crate::Readable for Hcirhs030Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs030::W`](W) writer structure"]
impl crate::Writable for Hcirhs030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS030 to value 0x1404_0000"]
impl crate::Resettable for Hcirhs030Spec {
    const RESET_VALUE: u32 = 0x1404_0000;
}
