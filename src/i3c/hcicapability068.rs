#[doc = "Register `HCICAPABILITY068` reader"]
pub type R = crate::R<Hcicapability068Spec>;
#[doc = "Register `HCICAPABILITY068` writer"]
pub type W = crate::W<Hcicapability068Spec>;
#[doc = "Field `REGLISTSIZE` reader - REG_LIST_SIZE"]
pub type ReglistsizeR = crate::FieldReader<u16>;
#[doc = "Field `REGLISTSIZE` writer - REG_LIST_SIZE"]
pub type ReglistsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGBLP` reader - REG_BLP"]
pub type RegblpR = crate::BitReader;
#[doc = "Field `REGBLP` writer - REG_BLP"]
pub type RegblpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - REG_LIST_SIZE"]
    #[inline(always)]
    pub fn reglistsize(&self) -> ReglistsizeR {
        ReglistsizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - REG_BLP"]
    #[inline(always)]
    pub fn regblp(&self) -> RegblpR {
        RegblpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_LIST_SIZE"]
    #[inline(always)]
    pub fn reglistsize(&mut self) -> ReglistsizeW<Hcicapability068Spec> {
        ReglistsizeW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_BLP"]
    #[inline(always)]
    pub fn regblp(&mut self) -> RegblpW<Hcicapability068Spec> {
        RegblpW::new(self, 31)
    }
}
#[doc = "DEV\\_CTX\\_SG\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability068Spec;
impl crate::RegisterSpec for Hcicapability068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability068::R`](R) reader structure"]
impl crate::Readable for Hcicapability068Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability068::W`](W) writer structure"]
impl crate::Writable for Hcicapability068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY068 to value 0"]
impl crate::Resettable for Hcicapability068Spec {}
