#[doc = "Register `SCUE34` reader"]
pub type R = crate::R<Scue34Spec>;
#[doc = "Register `SCUE34` writer"]
pub type W = crate::W<Scue34Spec>;
#[doc = "Field `SCUREGLOCK680` reader - SCU_REG_LOCK_680"]
pub type Scureglock680R = crate::BitReader;
#[doc = "Field `SCUREGLOCK680` writer - SCU_REG_LOCK_680"]
pub type Scureglock680W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK684` reader - SCU_REG_LOCK_684"]
pub type Scureglock684R = crate::BitReader;
#[doc = "Field `SCUREGLOCK684` writer - SCU_REG_LOCK_684"]
pub type Scureglock684W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK688` reader - SCU_REG_LOCK_688"]
pub type Scureglock688R = crate::BitReader;
#[doc = "Field `SCUREGLOCK688` writer - SCU_REG_LOCK_688"]
pub type Scureglock688W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK68C` reader - SCU_REG_LOCK_68C"]
pub type Scureglock68cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK68C` writer - SCU_REG_LOCK_68C"]
pub type Scureglock68cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK690` reader - SCU_REG_LOCK_690"]
pub type Scureglock690R = crate::BitReader;
#[doc = "Field `SCUREGLOCK690` writer - SCU_REG_LOCK_690"]
pub type Scureglock690W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK694` reader - SCU_REG_LOCK_694"]
pub type Scureglock694R = crate::BitReader;
#[doc = "Field `SCUREGLOCK694` writer - SCU_REG_LOCK_694"]
pub type Scureglock694W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK698` reader - SCU_REG_LOCK_698"]
pub type Scureglock698R = crate::BitReader;
#[doc = "Field `SCUREGLOCK698` writer - SCU_REG_LOCK_698"]
pub type Scureglock698W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK69C` reader - SCU_REG_LOCK_69C"]
pub type Scureglock69cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK69C` writer - SCU_REG_LOCK_69C"]
pub type Scureglock69cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK6A0` reader - SCU_REG_LOCK_6A0"]
pub type Scureglock6a0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK6A0` writer - SCU_REG_LOCK_6A0"]
pub type Scureglock6a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK6FC` reader - SCU_REG_LOCK_6FC"]
pub type Scureglock6fcR = crate::BitReader;
#[doc = "Field `SCUREGLOCK6FC` writer - SCU_REG_LOCK_6FC"]
pub type Scureglock6fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_680"]
    #[inline(always)]
    pub fn scureglock680(&self) -> Scureglock680R {
        Scureglock680R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_684"]
    #[inline(always)]
    pub fn scureglock684(&self) -> Scureglock684R {
        Scureglock684R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_688"]
    #[inline(always)]
    pub fn scureglock688(&self) -> Scureglock688R {
        Scureglock688R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_68C"]
    #[inline(always)]
    pub fn scureglock68c(&self) -> Scureglock68cR {
        Scureglock68cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_690"]
    #[inline(always)]
    pub fn scureglock690(&self) -> Scureglock690R {
        Scureglock690R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_694"]
    #[inline(always)]
    pub fn scureglock694(&self) -> Scureglock694R {
        Scureglock694R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_698"]
    #[inline(always)]
    pub fn scureglock698(&self) -> Scureglock698R {
        Scureglock698R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_69C"]
    #[inline(always)]
    pub fn scureglock69c(&self) -> Scureglock69cR {
        Scureglock69cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_6A0"]
    #[inline(always)]
    pub fn scureglock6a0(&self) -> Scureglock6a0R {
        Scureglock6a0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_LOCK_6FC"]
    #[inline(always)]
    pub fn scureglock6fc(&self) -> Scureglock6fcR {
        Scureglock6fcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_680"]
    #[inline(always)]
    pub fn scureglock680(&mut self) -> Scureglock680W<Scue34Spec> {
        Scureglock680W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_684"]
    #[inline(always)]
    pub fn scureglock684(&mut self) -> Scureglock684W<Scue34Spec> {
        Scureglock684W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_688"]
    #[inline(always)]
    pub fn scureglock688(&mut self) -> Scureglock688W<Scue34Spec> {
        Scureglock688W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_68C"]
    #[inline(always)]
    pub fn scureglock68c(&mut self) -> Scureglock68cW<Scue34Spec> {
        Scureglock68cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_690"]
    #[inline(always)]
    pub fn scureglock690(&mut self) -> Scureglock690W<Scue34Spec> {
        Scureglock690W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_694"]
    #[inline(always)]
    pub fn scureglock694(&mut self) -> Scureglock694W<Scue34Spec> {
        Scureglock694W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_698"]
    #[inline(always)]
    pub fn scureglock698(&mut self) -> Scureglock698W<Scue34Spec> {
        Scureglock698W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_69C"]
    #[inline(always)]
    pub fn scureglock69c(&mut self) -> Scureglock69cW<Scue34Spec> {
        Scureglock69cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_6A0"]
    #[inline(always)]
    pub fn scureglock6a0(&mut self) -> Scureglock6a0W<Scue34Spec> {
        Scureglock6a0W::new(self, 8)
    }
    #[doc = "Bit 31 - SCU_REG_LOCK_6FC"]
    #[inline(always)]
    pub fn scureglock6fc(&mut self) -> Scureglock6fcW<Scue34Spec> {
        Scureglock6fcW::new(self, 31)
    }
}
#[doc = "Write Protection 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue34Spec;
impl crate::RegisterSpec for Scue34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue34::R`](R) reader structure"]
impl crate::Readable for Scue34Spec {}
#[doc = "`write(|w| ..)` method takes [`scue34::W`](W) writer structure"]
impl crate::Writable for Scue34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE34 to value 0"]
impl crate::Resettable for Scue34Spec {}
