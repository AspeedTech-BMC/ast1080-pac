#[doc = "Register `HCIRHS05C` reader"]
pub type R = crate::R<Hcirhs05cSpec>;
#[doc = "Register `HCIRHS05C` writer"]
pub type W = crate::W<Hcirhs05cSpec>;
#[doc = "Field `REGCRDEQPTR` reader - REG_CR_DEQ_PTR"]
pub type RegcrdeqptrR = crate::FieldReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGIBIENQPTR` reader - REG_IBI_ENQ_PTR"]
pub type RegibienqptrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - REG_CR_DEQ_PTR"]
    #[inline(always)]
    pub fn regcrdeqptr(&self) -> RegcrdeqptrR {
        RegcrdeqptrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_IBI_ENQ_PTR"]
    #[inline(always)]
    pub fn regibienqptr(&self) -> RegibienqptrR {
        RegibienqptrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {}
#[doc = "RH\\_OPERATION2\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs05cSpec;
impl crate::RegisterSpec for Hcirhs05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs05c::R`](R) reader structure"]
impl crate::Readable for Hcirhs05cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs05c::W`](W) writer structure"]
impl crate::Writable for Hcirhs05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS05C to value 0"]
impl crate::Resettable for Hcirhs05cSpec {}
