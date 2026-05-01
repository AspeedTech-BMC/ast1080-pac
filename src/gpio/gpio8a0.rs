#[doc = "Register `GPIO8A0` reader"]
pub type R = crate::R<Gpio8a0Spec>;
#[doc = "Register `GPIO8A0` writer"]
pub type W = crate::W<Gpio8a0Spec>;
#[doc = "Field `GPIO144WrPrivilegeOfMaster` reader - GPIO144 Write Privilege of Master"]
pub type Gpio144wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO144WrPrivilegeOfMaster` writer - GPIO144 Write Privilege of Master"]
pub type Gpio144wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO145WrPrivilegeOfMaster` reader - GPIO145 Write Privilege of Master"]
pub type Gpio145wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO145WrPrivilegeOfMaster` writer - GPIO145 Write Privilege of Master"]
pub type Gpio145wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO146WrPrivilegeOfMaster` reader - GPIO146 Write Privilege of Master"]
pub type Gpio146wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO146WrPrivilegeOfMaster` writer - GPIO146 Write Privilege of Master"]
pub type Gpio146wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO147WrPrivilegeOfMaster` reader - GPIO147 Write Privilege of Master"]
pub type Gpio147wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO147WrPrivilegeOfMaster` writer - GPIO147 Write Privilege of Master"]
pub type Gpio147wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO144 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio144wr_privilege_of_master(&self) -> Gpio144wrPrivilegeOfMasterR {
        Gpio144wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO145 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio145wr_privilege_of_master(&self) -> Gpio145wrPrivilegeOfMasterR {
        Gpio145wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO146 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio146wr_privilege_of_master(&self) -> Gpio146wrPrivilegeOfMasterR {
        Gpio146wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO147 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio147wr_privilege_of_master(&self) -> Gpio147wrPrivilegeOfMasterR {
        Gpio147wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO144 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio144wr_privilege_of_master(&mut self) -> Gpio144wrPrivilegeOfMasterW<Gpio8a0Spec> {
        Gpio144wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO145 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio145wr_privilege_of_master(&mut self) -> Gpio145wrPrivilegeOfMasterW<Gpio8a0Spec> {
        Gpio145wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO146 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio146wr_privilege_of_master(&mut self) -> Gpio146wrPrivilegeOfMasterW<Gpio8a0Spec> {
        Gpio146wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO147 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio147wr_privilege_of_master(&mut self) -> Gpio147wrPrivilegeOfMasterW<Gpio8a0Spec> {
        Gpio147wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8a0Spec;
impl crate::RegisterSpec for Gpio8a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8a0::R`](R) reader structure"]
impl crate::Readable for Gpio8a0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8a0::W`](W) writer structure"]
impl crate::Writable for Gpio8a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8A0 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8a0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
