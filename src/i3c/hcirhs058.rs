#[doc = "Register `HCIRHS058` reader"]
pub type R = crate::R<Hcirhs058Spec>;
#[doc = "Register `HCIRHS058` writer"]
pub type W = crate::W<Hcirhs058Spec>;
#[doc = "Field `REGCRENQPTR` reader - REG_CR_ENQ_PTR"]
pub type RegcrenqptrR = crate::FieldReader;
#[doc = "Field `REGCRENQPTR` writer - REG_CR_ENQ_PTR"]
pub type RegcrenqptrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGCRSWDEQPTR` reader - REG_CR_SW_DEQ_PTR"]
pub type RegcrswdeqptrR = crate::FieldReader;
#[doc = "Field `REGCRSWDEQPTR` writer - REG_CR_SW_DEQ_PTR"]
pub type RegcrswdeqptrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGIBIDEQPTR` reader - REG_IBI_DEQ_PTR"]
pub type RegibideqptrR = crate::FieldReader;
#[doc = "Field `REGIBIDEQPTR` writer - REG_IBI_DEQ_PTR"]
pub type RegibideqptrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REG_CR_ENQ_PTR"]
    #[inline(always)]
    pub fn regcrenqptr(&self) -> RegcrenqptrR {
        RegcrenqptrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_CR_SW_DEQ_PTR"]
    #[inline(always)]
    pub fn regcrswdeqptr(&self) -> RegcrswdeqptrR {
        RegcrswdeqptrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_IBI_DEQ_PTR"]
    #[inline(always)]
    pub fn regibideqptr(&self) -> RegibideqptrR {
        RegibideqptrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_CR_ENQ_PTR"]
    #[inline(always)]
    pub fn regcrenqptr(&mut self) -> RegcrenqptrW<Hcirhs058Spec> {
        RegcrenqptrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_CR_SW_DEQ_PTR"]
    #[inline(always)]
    pub fn regcrswdeqptr(&mut self) -> RegcrswdeqptrW<Hcirhs058Spec> {
        RegcrswdeqptrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_IBI_DEQ_PTR"]
    #[inline(always)]
    pub fn regibideqptr(&mut self) -> RegibideqptrW<Hcirhs058Spec> {
        RegibideqptrW::new(self, 16)
    }
}
#[doc = "RH\\_OPERATION1\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs058Spec;
impl crate::RegisterSpec for Hcirhs058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs058::R`](R) reader structure"]
impl crate::Readable for Hcirhs058Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs058::W`](W) writer structure"]
impl crate::Writable for Hcirhs058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS058 to value 0"]
impl crate::Resettable for Hcirhs058Spec {}
