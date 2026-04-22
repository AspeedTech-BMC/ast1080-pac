#[doc = "Register `SPIF01C` reader"]
pub type R = crate::R<Spif01cSpec>;
#[doc = "Register `SPIF01C` writer"]
pub type W = crate::W<Spif01cSpec>;
#[doc = "Field `CS3BASE` reader - CS3_BASE"]
pub type Cs3baseR = crate::FieldReader<u16>;
#[doc = "Field `CS3BASE` writer - CS3_BASE"]
pub type Cs3baseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CS3_BASE"]
    #[inline(always)]
    pub fn cs3base(&self) -> Cs3baseR {
        Cs3baseR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CS3_BASE"]
    #[inline(always)]
    pub fn cs3base(&mut self) -> Cs3baseW<Spif01cSpec> {
        Cs3baseW::new(self, 0)
    }
}
#[doc = "SPIF\\_CSBASE3\n\nYou can [`read`](crate::Reg::read) this register and get [`spif01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif01cSpec;
impl crate::RegisterSpec for Spif01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif01c::R`](R) reader structure"]
impl crate::Readable for Spif01cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif01c::W`](W) writer structure"]
impl crate::Writable for Spif01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF01C to value 0"]
impl crate::Resettable for Spif01cSpec {}
