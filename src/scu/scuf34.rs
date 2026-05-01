#[doc = "Register `SCUF34` reader"]
pub type R = crate::R<Scuf34Spec>;
#[doc = "Register `SCUF34` writer"]
pub type W = crate::W<Scuf34Spec>;
#[doc = "Field `SCUREGRST680` reader - SCU_REG_RST_680"]
pub type Scuregrst680R = crate::BitReader;
#[doc = "Field `SCUREGRST680` writer - SCU_REG_RST_680"]
pub type Scuregrst680W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST684` reader - SCU_REG_RST_684"]
pub type Scuregrst684R = crate::BitReader;
#[doc = "Field `SCUREGRST684` writer - SCU_REG_RST_684"]
pub type Scuregrst684W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST688` reader - SCU_REG_RST_688"]
pub type Scuregrst688R = crate::BitReader;
#[doc = "Field `SCUREGRST688` writer - SCU_REG_RST_688"]
pub type Scuregrst688W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST68C` reader - SCU_REG_RST_68C"]
pub type Scuregrst68cR = crate::BitReader;
#[doc = "Field `SCUREGRST68C` writer - SCU_REG_RST_68C"]
pub type Scuregrst68cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST690` reader - SCU_REG_RST_690"]
pub type Scuregrst690R = crate::BitReader;
#[doc = "Field `SCUREGRST690` writer - SCU_REG_RST_690"]
pub type Scuregrst690W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST694` reader - SCU_REG_RST_694"]
pub type Scuregrst694R = crate::BitReader;
#[doc = "Field `SCUREGRST694` writer - SCU_REG_RST_694"]
pub type Scuregrst694W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST698` reader - SCU_REG_RST_698"]
pub type Scuregrst698R = crate::BitReader;
#[doc = "Field `SCUREGRST698` writer - SCU_REG_RST_698"]
pub type Scuregrst698W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST69C` reader - SCU_REG_RST_69C"]
pub type Scuregrst69cR = crate::BitReader;
#[doc = "Field `SCUREGRST69C` writer - SCU_REG_RST_69C"]
pub type Scuregrst69cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST6A0` reader - SCU_REG_RST_6A0"]
pub type Scuregrst6a0R = crate::BitReader;
#[doc = "Field `SCUREGRST6A0` writer - SCU_REG_RST_6A0"]
pub type Scuregrst6a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST6FC` reader - SCU_REG_RST_6FC"]
pub type Scuregrst6fcR = crate::BitReader;
#[doc = "Field `SCUREGRST6FC` writer - SCU_REG_RST_6FC"]
pub type Scuregrst6fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_680"]
    #[inline(always)]
    pub fn scuregrst680(&self) -> Scuregrst680R {
        Scuregrst680R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_684"]
    #[inline(always)]
    pub fn scuregrst684(&self) -> Scuregrst684R {
        Scuregrst684R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_RST_688"]
    #[inline(always)]
    pub fn scuregrst688(&self) -> Scuregrst688R {
        Scuregrst688R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_RST_68C"]
    #[inline(always)]
    pub fn scuregrst68c(&self) -> Scuregrst68cR {
        Scuregrst68cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_RST_690"]
    #[inline(always)]
    pub fn scuregrst690(&self) -> Scuregrst690R {
        Scuregrst690R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_RST_694"]
    #[inline(always)]
    pub fn scuregrst694(&self) -> Scuregrst694R {
        Scuregrst694R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_RST_698"]
    #[inline(always)]
    pub fn scuregrst698(&self) -> Scuregrst698R {
        Scuregrst698R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_RST_69C"]
    #[inline(always)]
    pub fn scuregrst69c(&self) -> Scuregrst69cR {
        Scuregrst69cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_RST_6A0"]
    #[inline(always)]
    pub fn scuregrst6a0(&self) -> Scuregrst6a0R {
        Scuregrst6a0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_RST_6FC"]
    #[inline(always)]
    pub fn scuregrst6fc(&self) -> Scuregrst6fcR {
        Scuregrst6fcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_680"]
    #[inline(always)]
    pub fn scuregrst680(&mut self) -> Scuregrst680W<Scuf34Spec> {
        Scuregrst680W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_684"]
    #[inline(always)]
    pub fn scuregrst684(&mut self) -> Scuregrst684W<Scuf34Spec> {
        Scuregrst684W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_RST_688"]
    #[inline(always)]
    pub fn scuregrst688(&mut self) -> Scuregrst688W<Scuf34Spec> {
        Scuregrst688W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_RST_68C"]
    #[inline(always)]
    pub fn scuregrst68c(&mut self) -> Scuregrst68cW<Scuf34Spec> {
        Scuregrst68cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_RST_690"]
    #[inline(always)]
    pub fn scuregrst690(&mut self) -> Scuregrst690W<Scuf34Spec> {
        Scuregrst690W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_RST_694"]
    #[inline(always)]
    pub fn scuregrst694(&mut self) -> Scuregrst694W<Scuf34Spec> {
        Scuregrst694W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_RST_698"]
    #[inline(always)]
    pub fn scuregrst698(&mut self) -> Scuregrst698W<Scuf34Spec> {
        Scuregrst698W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_RST_69C"]
    #[inline(always)]
    pub fn scuregrst69c(&mut self) -> Scuregrst69cW<Scuf34Spec> {
        Scuregrst69cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_RST_6A0"]
    #[inline(always)]
    pub fn scuregrst6a0(&mut self) -> Scuregrst6a0W<Scuf34Spec> {
        Scuregrst6a0W::new(self, 8)
    }
    #[doc = "Bit 31 - SCU_REG_RST_6FC"]
    #[inline(always)]
    pub fn scuregrst6fc(&mut self) -> Scuregrst6fcW<Scuf34Spec> {
        Scuregrst6fcW::new(self, 31)
    }
}
#[doc = "Reset Control 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf34Spec;
impl crate::RegisterSpec for Scuf34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf34::R`](R) reader structure"]
impl crate::Readable for Scuf34Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf34::W`](W) writer structure"]
impl crate::Writable for Scuf34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF34 to value 0"]
impl crate::Resettable for Scuf34Spec {}
