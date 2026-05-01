#[doc = "Register `HCIRHS000` reader"]
pub type R = crate::R<Hcirhs000Spec>;
#[doc = "Register `HCIRHS000` writer"]
pub type W = crate::W<Hcirhs000Spec>;
#[doc = "Field `REGMAXHEADERCOUNT` reader - REG_MAX_HEADER_COUNT"]
pub type RegmaxheadercountR = crate::FieldReader;
#[doc = "Field `REGMAXHEADERCOUNT` writer - REG_MAX_HEADER_COUNT"]
pub type RegmaxheadercountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REGMAXHEADERCOUNTCAPABILITY` reader - REG_MAX_HEADER_COUNT_CAPABILITY"]
pub type RegmaxheadercountcapabilityR = crate::FieldReader;
#[doc = "Field `REGHEADERSIZE` reader - REG_HEADER_SIZE"]
pub type RegheadersizeR = crate::FieldReader;
#[doc = "Field `REGPREAMBLESIZE` reader - REG_PREAMBLE_SIZE"]
pub type RegpreamblesizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - REG_MAX_HEADER_COUNT"]
    #[inline(always)]
    pub fn regmaxheadercount(&self) -> RegmaxheadercountR {
        RegmaxheadercountR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - REG_MAX_HEADER_COUNT_CAPABILITY"]
    #[inline(always)]
    pub fn regmaxheadercountcapability(&self) -> RegmaxheadercountcapabilityR {
        RegmaxheadercountcapabilityR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - REG_HEADER_SIZE"]
    #[inline(always)]
    pub fn regheadersize(&self) -> RegheadersizeR {
        RegheadersizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_PREAMBLE_SIZE"]
    #[inline(always)]
    pub fn regpreamblesize(&self) -> RegpreamblesizeR {
        RegpreamblesizeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - REG_MAX_HEADER_COUNT"]
    #[inline(always)]
    pub fn regmaxheadercount(&mut self) -> RegmaxheadercountW<Hcirhs000Spec> {
        RegmaxheadercountW::new(self, 0)
    }
}
#[doc = "RHS\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs000Spec;
impl crate::RegisterSpec for Hcirhs000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs000::R`](R) reader structure"]
impl crate::Readable for Hcirhs000Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs000::W`](W) writer structure"]
impl crate::Writable for Hcirhs000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS000 to value 0x0205_0010"]
impl crate::Resettable for Hcirhs000Spec {
    const RESET_VALUE: u32 = 0x0205_0010;
}
