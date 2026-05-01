#[doc = "Register `GPIO854` reader"]
pub type R = crate::R<Gpio854Spec>;
#[doc = "Register `GPIO854` writer"]
pub type W = crate::W<Gpio854Spec>;
#[doc = "Field `GPIO068WrPrivilegeOfMaster` reader - GPIO068 Write Privilege of Master"]
pub type Gpio068wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO068WrPrivilegeOfMaster` writer - GPIO068 Write Privilege of Master"]
pub type Gpio068wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO069WrPrivilegeOfMaster` reader - GPIO069 Write Privilege of Master"]
pub type Gpio069wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO069WrPrivilegeOfMaster` writer - GPIO069 Write Privilege of Master"]
pub type Gpio069wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO070WrPrivilegeOfMaster` reader - GPIO070 Write Privilege of Master"]
pub type Gpio070wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO070WrPrivilegeOfMaster` writer - GPIO070 Write Privilege of Master"]
pub type Gpio070wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO071WrPrivilegeOfMaster` reader - GPIO071 Write Privilege of Master"]
pub type Gpio071wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO071WrPrivilegeOfMaster` writer - GPIO071 Write Privilege of Master"]
pub type Gpio071wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO068 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio068wr_privilege_of_master(&self) -> Gpio068wrPrivilegeOfMasterR {
        Gpio068wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO069 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio069wr_privilege_of_master(&self) -> Gpio069wrPrivilegeOfMasterR {
        Gpio069wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO070 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio070wr_privilege_of_master(&self) -> Gpio070wrPrivilegeOfMasterR {
        Gpio070wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO071 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio071wr_privilege_of_master(&self) -> Gpio071wrPrivilegeOfMasterR {
        Gpio071wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO068 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio068wr_privilege_of_master(&mut self) -> Gpio068wrPrivilegeOfMasterW<Gpio854Spec> {
        Gpio068wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO069 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio069wr_privilege_of_master(&mut self) -> Gpio069wrPrivilegeOfMasterW<Gpio854Spec> {
        Gpio069wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO070 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio070wr_privilege_of_master(&mut self) -> Gpio070wrPrivilegeOfMasterW<Gpio854Spec> {
        Gpio070wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO071 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio071wr_privilege_of_master(&mut self) -> Gpio071wrPrivilegeOfMasterW<Gpio854Spec> {
        Gpio071wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio854::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio854::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio854Spec;
impl crate::RegisterSpec for Gpio854Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio854::R`](R) reader structure"]
impl crate::Readable for Gpio854Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio854::W`](W) writer structure"]
impl crate::Writable for Gpio854Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO854 to value 0xffff_ffff"]
impl crate::Resettable for Gpio854Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
